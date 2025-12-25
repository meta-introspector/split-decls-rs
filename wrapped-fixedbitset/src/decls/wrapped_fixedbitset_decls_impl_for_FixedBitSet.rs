use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FixedBitSet {
    /// Create a new empty **FixedBitSet**.
    pub const fn new() -> Self {
        FixedBitSet {
            data: NonNull::dangling(),
            capacity: 0,
            length: 0,
        }
    }
    /// Create a new **FixedBitSet** with a specific number of bits,
    /// all initially clear.
    pub fn with_capacity(bits: usize) -> Self {
        let (mut blocks, rem) = div_rem(bits, SimdBlock::BITS);
        blocks += (rem > 0) as usize;
        Self::from_blocks_and_len(vec![SimdBlock::NONE; blocks], bits)
    }
    #[inline]
    fn from_blocks_and_len(data: Vec<SimdBlock>, length: usize) -> Self {
        let (data, capacity, _) = vec_into_parts(data);
        FixedBitSet {
            data: data.cast(),
            capacity,
            length,
        }
    }
    /// Create a new **FixedBitSet** with a specific number of bits,
    /// initialized from provided blocks.
    ///
    /// If the blocks are not the exact size needed for the capacity
    /// they will be padded with zeros (if shorter) or truncated to
    /// the capacity (if longer).
    ///
    /// For example:
    /// ```
    /// let data = vec![4];
    /// let bs = fixedbitset::FixedBitSet::with_capacity_and_blocks(4, data);
    /// assert_eq!(format!("{:b}", bs), "0010");
    /// ```
    pub fn with_capacity_and_blocks<I: IntoIterator<Item = Block>>(
        bits: usize,
        blocks: I,
    ) -> Self {
        let mut bitset = Self::with_capacity(bits);
        for (subblock, value) in bitset.as_mut_slice().iter_mut().zip(blocks.into_iter())
        {
            *subblock = value;
        }
        bitset
    }
    /// Grow capacity to **bits**, all new bits initialized to zero
    #[inline]
    pub fn grow(&mut self, bits: usize) {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn do_grow(slf: &mut FixedBitSet, bits: usize) {
            unsafe { slf.grow_inner(bits, MaybeUninit::new(SimdBlock::NONE)) };
        }
        if bits > self.length {
            do_grow(self, bits);
        }
    }
    /// # Safety
    /// If `fill` is uninitialized, the memory must not be accessed and must be immediately
    /// written over
    #[inline(always)]
    unsafe fn grow_inner(&mut self, bits: usize, fill: MaybeUninit<SimdBlock>) {
        let mut data = unsafe {
            Vec::from_raw_parts(self.data.as_ptr(), self.simd_block_len(), self.capacity)
        };
        let (mut blocks, rem) = div_rem(bits, SimdBlock::BITS);
        blocks += (rem > 0) as usize;
        data.resize(blocks, fill);
        let (data, capacity, _) = vec_into_parts(data);
        self.data = data;
        self.capacity = capacity;
        self.length = bits;
    }
    #[inline]
    unsafe fn get_unchecked(&self, subblock: usize) -> &Block {
        &*self.data.as_ptr().cast::<Block>().add(subblock)
    }
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, subblock: usize) -> &mut Block {
        &mut *self.data.as_ptr().cast::<Block>().add(subblock)
    }
    #[inline]
    fn usize_len(&self) -> usize {
        let (mut blocks, rem) = div_rem(self.length, BITS);
        blocks += (rem > 0) as usize;
        blocks
    }
    #[inline]
    fn simd_block_len(&self) -> usize {
        let (mut blocks, rem) = div_rem(self.length, SimdBlock::BITS);
        blocks += (rem > 0) as usize;
        blocks
    }
    #[inline]
    fn batch_count_ones(blocks: impl IntoIterator<Item = Block>) -> usize {
        blocks.into_iter().map(|x| x.count_ones() as usize).sum()
    }
    #[inline]
    fn as_simd_slice(&self) -> &[SimdBlock] {
        unsafe {
            core::slice::from_raw_parts(self.data.as_ptr().cast(), self.simd_block_len())
        }
    }
    #[inline]
    fn as_mut_simd_slice(&mut self) -> &mut [SimdBlock] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.data.as_ptr().cast(),
                self.simd_block_len(),
            )
        }
    }
    #[inline]
    fn as_simd_slice_uninit(&self) -> &[MaybeUninit<SimdBlock>] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.simd_block_len()) }
    }
    #[inline]
    fn as_mut_simd_slice_uninit(&mut self) -> &mut [MaybeUninit<SimdBlock>] {
        unsafe {
            core::slice::from_raw_parts_mut(self.data.as_ptr(), self.simd_block_len())
        }
    }
    /// Grows the internal size of the bitset before inserting a bit
    ///
    /// Unlike `insert`, this cannot panic, but may allocate if the bit is outside of the existing buffer's range.
    ///
    /// This is faster than calling `grow` then `insert` in succession.
    #[inline]
    pub fn grow_and_insert(&mut self, bits: usize) {
        self.grow(bits + 1);
        let (blocks, rem) = div_rem(bits, BITS);
        unsafe {
            *self.get_unchecked_mut(blocks) |= 1 << rem;
        }
    }
    /// The length of the [`FixedBitSet`] in bits.
    ///
    /// Note: `len` includes both set and unset bits.
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let bitset = FixedBitSet::with_capacity(10);
    /// // there are 0 set bits, but 10 unset bits
    /// assert_eq!(bitset.len(), 10);
    /// ```
    /// `len` does not return the count of set bits. For that, use
    /// [`bitset.count_ones(..)`](FixedBitSet::count_ones) instead.
    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }
    /// `true` if the [`FixedBitSet`] is empty.
    ///
    /// Note that an "empty" `FixedBitSet` is a `FixedBitSet` with
    /// no bits (meaning: it's length is zero). If you want to check
    /// if all bits are unset, use [`FixedBitSet::is_clear`].
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let bitset = FixedBitSet::with_capacity(10);
    /// assert!(!bitset.is_empty());
    ///
    /// let bitset = FixedBitSet::with_capacity(0);
    /// assert!(bitset.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// `true` if all bits in the [`FixedBitSet`] are unset.
    ///
    /// As opposed to [`FixedBitSet::is_empty`], which is `true` only for
    /// sets without any bits, set or unset.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert!(bitset.is_clear());
    ///
    /// bitset.insert(2);
    /// assert!(!bitset.is_clear());
    /// ```
    ///
    /// This is equivalent to [`bitset.count_ones(..) == 0`](FixedBitSet::count_ones).
    #[inline]
    pub fn is_clear(&self) -> bool {
        self.as_simd_slice().iter().all(|block| block.is_empty())
    }
    /// Finds the lowest set bit in the bitset.
    ///
    /// Returns `None` if there aren't any set bits.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert_eq!(bitset.minimum(), None);
    ///
    /// bitset.insert(2);
    /// assert_eq!(bitset.minimum(), Some(2));
    /// bitset.insert(8);
    /// assert_eq!(bitset.minimum(), Some(2));
    /// ```
    #[inline]
    pub fn minimum(&self) -> Option<usize> {
        let (block_idx, block) = self
            .as_simd_slice()
            .iter()
            .enumerate()
            .find(|&(_, block)| !block.is_empty())?;
        let mut inner = 0;
        let mut trailing = 0;
        for subblock in block.into_usize_array() {
            if subblock != 0 {
                trailing = subblock.trailing_zeros() as usize;
                break;
            } else {
                inner += BITS;
            }
        }
        Some(block_idx * SimdBlock::BITS + inner + trailing)
    }
    /// Finds the highest set bit in the bitset.
    ///
    /// Returns `None` if there aren't any set bits.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert_eq!(bitset.maximum(), None);
    ///
    /// bitset.insert(8);
    /// assert_eq!(bitset.maximum(), Some(8));
    /// bitset.insert(2);
    /// assert_eq!(bitset.maximum(), Some(8));
    /// ```
    #[inline]
    pub fn maximum(&self) -> Option<usize> {
        let (block_idx, block) = self
            .as_simd_slice()
            .iter()
            .rev()
            .enumerate()
            .find(|&(_, block)| !block.is_empty())?;
        let mut inner = 0;
        let mut leading = 0;
        for subblock in block.into_usize_array().iter().rev() {
            if *subblock != 0 {
                leading = subblock.leading_zeros() as usize;
                break;
            } else {
                inner += BITS;
            }
        }
        let max = self.simd_block_len() * SimdBlock::BITS;
        Some(max - block_idx * SimdBlock::BITS - inner - leading - 1)
    }
    /// `true` if all bits in the [`FixedBitSet`] are set.
    ///
    /// ```
    /// # use fixedbitset::FixedBitSet;
    /// let mut bitset = FixedBitSet::with_capacity(10);
    /// assert!(!bitset.is_full());
    ///
    /// bitset.insert_range(..);
    /// assert!(bitset.is_full());
    /// ```
    ///
    /// This is equivalent to [`bitset.count_ones(..) == bitset.len()`](FixedBitSet::count_ones).
    #[inline]
    pub fn is_full(&self) -> bool {
        self.contains_all_in_range(..)
    }
    /// Return **true** if the bit is enabled in the **FixedBitSet**,
    /// **false** otherwise.
    ///
    /// Note: bits outside the capacity are always disabled.
    ///
    /// Note: Also available with index syntax: `bitset[bit]`.
    #[inline]
    pub fn contains(&self, bit: usize) -> bool {
        if bit < self.length { unsafe { self.contains_unchecked(bit) } } else { false }
    }
    /// Return **true** if the bit is enabled in the **FixedBitSet**,
    /// **false** otherwise.
    ///
    /// Note: unlike `contains`, calling this with an invalid `bit`
    /// is undefined behavior.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn contains_unchecked(&self, bit: usize) -> bool {
        let (block, i) = div_rem(bit, BITS);
        (self.get_unchecked(block) & (1 << i)) != 0
    }
    /// Clear all bits.
    #[inline]
    pub fn clear(&mut self) {
        for elt in self.as_mut_simd_slice().iter_mut() {
            *elt = SimdBlock::NONE;
        }
    }
    /// Enable `bit`.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn insert(&mut self, bit: usize) {
        assert!(
            bit < self.length, "insert at index {} exceeds fixedbitset size {}", bit,
            self.length
        );
        unsafe {
            self.insert_unchecked(bit);
        }
    }
    /// Enable `bit` without any length checks.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn insert_unchecked(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, BITS);
        unsafe {
            *self.get_unchecked_mut(block) |= 1 << i;
        }
    }
    /// Disable `bit`.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn remove(&mut self, bit: usize) {
        assert!(
            bit < self.length, "remove at index {} exceeds fixedbitset size {}", bit,
            self.length
        );
        unsafe {
            self.remove_unchecked(bit);
        }
    }
    /// Disable `bit` without any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn remove_unchecked(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, BITS);
        unsafe {
            *self.get_unchecked_mut(block) &= !(1 << i);
        }
    }
    /// Enable `bit`, and return its previous value.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn put(&mut self, bit: usize) -> bool {
        assert!(
            bit < self.length, "put at index {} exceeds fixedbitset size {}", bit, self
            .length
        );
        unsafe { self.put_unchecked(bit) }
    }
    /// Enable `bit`, and return its previous value without doing any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn put_unchecked(&mut self, bit: usize) -> bool {
        let (block, i) = div_rem(bit, BITS);
        unsafe {
            let word = self.get_unchecked_mut(block);
            let prev = *word & (1 << i) != 0;
            *word |= 1 << i;
            prev
        }
    }
    /// Toggle `bit` (inverting its state).
    ///
    /// ***Panics*** if **bit** is out of bounds
    #[inline]
    pub fn toggle(&mut self, bit: usize) {
        assert!(
            bit < self.length, "toggle at index {} exceeds fixedbitset size {}", bit,
            self.length
        );
        unsafe {
            self.toggle_unchecked(bit);
        }
    }
    /// Toggle `bit` (inverting its state) without any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn toggle_unchecked(&mut self, bit: usize) {
        let (block, i) = div_rem(bit, BITS);
        unsafe {
            *self.get_unchecked_mut(block) ^= 1 << i;
        }
    }
    /// Sets a bit to the provided `enabled` value.
    ///
    /// **Panics** if **bit** is out of bounds.
    #[inline]
    pub fn set(&mut self, bit: usize, enabled: bool) {
        assert!(
            bit < self.length, "set at index {} exceeds fixedbitset size {}", bit, self
            .length
        );
        unsafe {
            self.set_unchecked(bit, enabled);
        }
    }
    /// Sets a bit to the provided `enabled` value without doing any bounds checking.
    ///
    /// # Safety
    /// `bit` must be less than `self.len()`
    #[inline]
    pub unsafe fn set_unchecked(&mut self, bit: usize, enabled: bool) {
        let (block, i) = div_rem(bit, BITS);
        let elt = unsafe { self.get_unchecked_mut(block) };
        if enabled {
            *elt |= 1 << i;
        } else {
            *elt &= !(1 << i);
        }
    }
    /// Copies boolean value from specified bit to the specified bit.
    ///
    /// If `from` is out-of-bounds, `to` will be unset.
    ///
    /// **Panics** if **to** is out of bounds.
    #[inline]
    pub fn copy_bit(&mut self, from: usize, to: usize) {
        assert!(
            to < self.length, "copy to index {} exceeds fixedbitset size {}", to, self
            .length
        );
        let enabled = self.contains(from);
        unsafe { self.set_unchecked(to, enabled) };
    }
    /// Copies boolean value from specified bit to the specified bit.
    ///
    /// Note: unlike `copy_bit`, calling this with an invalid `from`
    /// is undefined behavior.
    ///
    /// # Safety
    /// `to` must both be less than `self.len()`
    #[inline]
    pub unsafe fn copy_bit_unchecked(&mut self, from: usize, to: usize) {
        let enabled = self.contains_unchecked(from);
        self.set_unchecked(to, enabled);
    }
    /// Count the number of set bits in the given bit range.
    ///
    /// This function is potentially much faster than using `ones(other).count()`.
    /// Use `..` to count the whole content of the bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn count_ones<T: IndexRange>(&self, range: T) -> usize {
        Self::batch_count_ones(
            Masks::new(range, self.length)
                .map(|(block, mask)| { unsafe { *self.get_unchecked(block) & mask } }),
        )
    }
    /// Count the number of unset bits in the given bit range.
    ///
    /// This function is potentially much faster than using `zeroes(other).count()`.
    /// Use `..` to count the whole content of the bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn count_zeroes<T: IndexRange>(&self, range: T) -> usize {
        Self::batch_count_ones(
            Masks::new(range, self.length)
                .map(|(block, mask)| { unsafe { !*self.get_unchecked(block) & mask } }),
        )
    }
    /// Sets every bit in the given range to the given state (`enabled`)
    ///
    /// Use `..` to set the whole bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn set_range<T: IndexRange>(&mut self, range: T, enabled: bool) {
        if enabled {
            self.insert_range(range);
        } else {
            self.remove_range(range);
        }
    }
    /// Enables every bit in the given range.
    ///
    /// Use `..` to make the whole bitset ones.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn insert_range<T: IndexRange>(&mut self, range: T) {
        for (block, mask) in Masks::new(range, self.length) {
            let block = unsafe { self.get_unchecked_mut(block) };
            *block |= mask;
        }
    }
    /// Disables every bit in the given range.
    ///
    /// Use `..` to make the whole bitset ones.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn remove_range<T: IndexRange>(&mut self, range: T) {
        for (block, mask) in Masks::new(range, self.length) {
            let block = unsafe { self.get_unchecked_mut(block) };
            *block &= !mask;
        }
    }
    /// Toggles (inverts) every bit in the given range.
    ///
    /// Use `..` to toggle the whole bitset.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn toggle_range<T: IndexRange>(&mut self, range: T) {
        for (block, mask) in Masks::new(range, self.length) {
            let block = unsafe { self.get_unchecked_mut(block) };
            *block ^= mask;
        }
    }
    /// Checks if the bitset contains every bit in the given range.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn contains_all_in_range<T: IndexRange>(&self, range: T) -> bool {
        for (block, mask) in Masks::new(range, self.length) {
            let block = unsafe { self.get_unchecked(block) };
            if block & mask != mask {
                return false;
            }
        }
        true
    }
    /// Checks if the bitset contains at least one set bit in the given range.
    ///
    /// **Panics** if the range extends past the end of the bitset.
    #[inline]
    pub fn contains_any_in_range<T: IndexRange>(&self, range: T) -> bool {
        for (block, mask) in Masks::new(range, self.length) {
            let block = unsafe { self.get_unchecked(block) };
            if block & mask != 0 {
                return true;
            }
        }
        false
    }
    /// View the bitset as a slice of `Block` blocks
    #[inline]
    pub fn as_slice(&self) -> &[Block] {
        unsafe {
            let ptr = self.data.as_ptr().cast::<Block>();
            core::slice::from_raw_parts(ptr, self.usize_len())
        }
    }
    /// View the bitset as a mutable slice of `Block` blocks. Writing past the bitlength in the last
    /// will cause `contains` to return potentially incorrect results for bits past the bitlength.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Block] {
        unsafe {
            let ptr = self.data.as_ptr().cast::<Block>();
            core::slice::from_raw_parts_mut(ptr, self.usize_len())
        }
    }
    /// Iterates over all enabled bits.
    ///
    /// Iterator element is the index of the `1` bit, type `usize`.
    #[inline]
    pub fn ones(&self) -> Ones<'_> {
        match self.as_slice().split_first() {
            Some((&first_block, rem)) => {
                let (&last_block, rem) = rem.split_last().unwrap_or((&0, rem));
                Ones {
                    bitset_front: first_block,
                    bitset_back: last_block,
                    block_idx_front: 0,
                    block_idx_back: (1 + rem.len()) * BITS,
                    remaining_blocks: rem.iter(),
                }
            }
            None => {
                Ones {
                    bitset_front: 0,
                    bitset_back: 0,
                    block_idx_front: 0,
                    block_idx_back: 0,
                    remaining_blocks: [].iter(),
                }
            }
        }
    }
    /// Iterates over all enabled bits.
    ///
    /// Iterator element is the index of the `1` bit, type `usize`.
    /// Unlike `ones`, this function consumes the `FixedBitset`.
    pub fn into_ones(self) -> IntoOnes {
        let ptr = self.data.as_ptr().cast();
        let len = self.simd_block_len() * SimdBlock::USIZE_COUNT;
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        let data: Vec<SimdBlock> = unsafe {
            Vec::from_raw_parts(
                self.data.as_ptr().cast(),
                self.simd_block_len(),
                self.capacity,
            )
        };
        let mut iter = slice.iter().copied();
        core::mem::forget(self);
        IntoOnes {
            bitset_front: iter.next().unwrap_or(0),
            bitset_back: iter.next_back().unwrap_or(0),
            block_idx_front: 0,
            block_idx_back: len.saturating_sub(1) * BITS,
            remaining_blocks: iter,
            _buf: data,
        }
    }
    /// Iterates over all disabled bits.
    ///
    /// Iterator element is the index of the `0` bit, type `usize`.
    #[inline]
    pub fn zeroes(&self) -> Zeroes<'_> {
        match self.as_slice().split_first() {
            Some((&block, rem)) => {
                Zeroes {
                    bitset: !block,
                    block_idx: 0,
                    len: self.len(),
                    remaining_blocks: rem.iter(),
                }
            }
            None => {
                Zeroes {
                    bitset: !0,
                    block_idx: 0,
                    len: self.len(),
                    remaining_blocks: [].iter(),
                }
            }
        }
    }
    /// Returns a lazy iterator over the intersection of two `FixedBitSet`s
    pub fn intersection<'a>(&'a self, other: &'a FixedBitSet) -> Intersection<'a> {
        Intersection {
            iter: self.ones(),
            other,
        }
    }
    /// Returns a lazy iterator over the union of two `FixedBitSet`s.
    pub fn union<'a>(&'a self, other: &'a FixedBitSet) -> Union<'a> {
        Union {
            iter: self.ones().chain(other.difference(self)),
        }
    }
    /// Returns a lazy iterator over the difference of two `FixedBitSet`s. The difference of `a`
    /// and `b` is the elements of `a` which are not in `b`.
    pub fn difference<'a>(&'a self, other: &'a FixedBitSet) -> Difference<'a> {
        Difference {
            iter: self.ones(),
            other,
        }
    }
    /// Returns a lazy iterator over the symmetric difference of two `FixedBitSet`s.
    /// The symmetric difference of `a` and `b` is the elements of one, but not both, sets.
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a FixedBitSet,
    ) -> SymmetricDifference<'a> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }
    /// In-place union of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity may be increased to match `other`'s.
    pub fn union_with(&mut self, other: &FixedBitSet) {
        if other.len() >= self.len() {
            self.grow(other.len());
        }
        self.as_mut_simd_slice()
            .iter_mut()
            .zip(other.as_simd_slice().iter())
            .for_each(|(x, y)| *x |= *y);
    }
    /// In-place intersection of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity will remain the same as before.
    pub fn intersect_with(&mut self, other: &FixedBitSet) {
        let me = self.as_mut_simd_slice();
        let other = other.as_simd_slice();
        me.iter_mut()
            .zip(other.iter())
            .for_each(|(x, y)| {
                *x &= *y;
            });
        let mn = core::cmp::min(me.len(), other.len());
        for wd in &mut me[mn..] {
            *wd = SimdBlock::NONE;
        }
    }
    /// In-place difference of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity will remain the same as before.
    pub fn difference_with(&mut self, other: &FixedBitSet) {
        self.as_mut_simd_slice()
            .iter_mut()
            .zip(other.as_simd_slice().iter())
            .for_each(|(x, y)| {
                *x &= !*y;
            });
    }
    /// In-place symmetric difference of two `FixedBitSet`s.
    ///
    /// On calling this method, `self`'s capacity may be increased to match `other`'s.
    pub fn symmetric_difference_with(&mut self, other: &FixedBitSet) {
        if other.len() >= self.len() {
            self.grow(other.len());
        }
        self.as_mut_simd_slice()
            .iter_mut()
            .zip(other.as_simd_slice().iter())
            .for_each(|(x, y)| {
                *x ^= *y;
            });
    }
    /// Computes how many bits would be set in the union between two bitsets.
    ///
    /// This is potentially much faster than using `union(other).count()`. Unlike
    /// other methods like using [`union_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn union_count(&self, other: &FixedBitSet) -> usize {
        let me = self.as_slice();
        let other = other.as_slice();
        let count = Self::batch_count_ones(
            me.iter().zip(other.iter()).map(|(x, y)| *x | *y),
        );
        match other.len().cmp(&me.len()) {
            Ordering::Greater => {
                count + Self::batch_count_ones(other[me.len()..].iter().copied())
            }
            Ordering::Less => {
                count + Self::batch_count_ones(me[other.len()..].iter().copied())
            }
            Ordering::Equal => count,
        }
    }
    /// Computes how many bits would be set in the intersection between two bitsets.
    ///
    /// This is potentially much faster than using `intersection(other).count()`. Unlike
    /// other methods like using [`intersect_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn intersection_count(&self, other: &FixedBitSet) -> usize {
        Self::batch_count_ones(
            self.as_slice().iter().zip(other.as_slice()).map(|(x, y)| *x & *y),
        )
    }
    /// Computes how many bits would be set in the difference between two bitsets.
    ///
    /// This is potentially much faster than using `difference(other).count()`. Unlike
    /// other methods like using [`difference_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn difference_count(&self, other: &FixedBitSet) -> usize {
        Self::batch_count_ones(
            self.as_slice().iter().zip(other.as_slice().iter()).map(|(x, y)| *x & !*y),
        )
            + Self::batch_count_ones(
                self.as_slice().iter().skip(other.as_slice().len()).copied(),
            )
    }
    /// Computes how many bits would be set in the symmetric difference between two bitsets.
    ///
    /// This is potentially much faster than using `symmetric_difference(other).count()`. Unlike
    /// other methods like using [`symmetric_difference_with`] followed by [`count_ones`], this
    /// does not mutate in place or require separate allocations.
    #[inline]
    pub fn symmetric_difference_count(&self, other: &FixedBitSet) -> usize {
        let me = self.as_slice();
        let other = other.as_slice();
        let count = Self::batch_count_ones(
            me.iter().zip(other.iter()).map(|(x, y)| *x ^ *y),
        );
        match other.len().cmp(&me.len()) {
            Ordering::Greater => {
                count + Self::batch_count_ones(other[me.len()..].iter().copied())
            }
            Ordering::Less => {
                count + Self::batch_count_ones(me[other.len()..].iter().copied())
            }
            Ordering::Equal => count,
        }
    }
    /// Returns `true` if `self` has no elements in common with `other`. This
    /// is equivalent to checking for an empty intersection.
    pub fn is_disjoint(&self, other: &FixedBitSet) -> bool {
        self.as_simd_slice()
            .iter()
            .zip(other.as_simd_slice())
            .all(|(x, y)| (*x & *y).is_empty())
    }
    /// Returns `true` if the set is a subset of another, i.e. `other` contains
    /// at least all the values in `self`.
    pub fn is_subset(&self, other: &FixedBitSet) -> bool {
        let me = self.as_simd_slice();
        let other = other.as_simd_slice();
        me.iter().zip(other.iter()).all(|(x, y)| x.andnot(*y).is_empty())
            && me.iter().skip(other.len()).all(|x| x.is_empty())
    }
    /// Returns `true` if the set is a superset of another, i.e. `self` contains
    /// at least all the values in `other`.
    pub fn is_superset(&self, other: &FixedBitSet) -> bool {
        other.is_subset(self)
    }
}

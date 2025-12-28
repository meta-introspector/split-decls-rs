macro_rules! deps {
    () => {
        BitMask!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [allow (clippy :: use_self)] impl BitMask { # [doc = " Returns a new `BitMask` with all bits inverted."] # [inline] # [must_use] # [allow (dead_code)] pub (crate) fn invert (self) -> Self { BitMask (self . 0 ^ BITMASK_MASK) } # [doc = " Returns a new `BitMask` with the lowest bit removed."] # [inline] # [must_use] fn remove_lowest_bit (self) -> Self { BitMask (self . 0 & (self . 0 - 1)) } # [doc = " Returns whether the `BitMask` has at least one set bit."] # [inline] pub (crate) fn any_bit_set (self) -> bool { self . 0 != 0 } # [doc = " Returns the first set bit in the `BitMask`, if there is one."] # [inline] pub (crate) fn lowest_set_bit (self) -> Option < usize > { if let Some (nonzero) = NonZeroBitMaskWord :: new (self . 0) { Some (Self :: nonzero_trailing_zeros (nonzero)) } else { None } } # [doc = " Returns the number of trailing zeroes in the `BitMask`."] # [inline] pub (crate) fn trailing_zeros (self) -> usize { if cfg ! (target_arch = "arm") && BITMASK_STRIDE % 8 == 0 { self . 0 . swap_bytes () . leading_zeros () as usize / BITMASK_STRIDE } else { self . 0 . trailing_zeros () as usize / BITMASK_STRIDE } } # [doc = " Same as above but takes a `NonZeroBitMaskWord`."] # [inline] fn nonzero_trailing_zeros (nonzero : NonZeroBitMaskWord) -> usize { if cfg ! (target_arch = "arm") && BITMASK_STRIDE % 8 == 0 { let swapped = unsafe { NonZeroBitMaskWord :: new_unchecked (nonzero . get () . swap_bytes ()) } ; swapped . leading_zeros () as usize / BITMASK_STRIDE } else { nonzero . trailing_zeros () as usize / BITMASK_STRIDE } } # [doc = " Returns the number of leading zeroes in the `BitMask`."] # [inline] pub (crate) fn leading_zeros (self) -> usize { self . 0 . leading_zeros () as usize / BITMASK_STRIDE } }
    };
}

impl_8!();
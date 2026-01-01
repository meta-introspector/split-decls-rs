// SRC: ../rust/compiler/rustc_metadata/src/rmeta/table.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=2 | LINES=8 */
use crate::rustc_complete::def::CtorOf;
use crate::rustc_index::Idx;

use crate::rmeta::*;

pub(super) trait IsDefault: Default {
    fn is_default(&self) -> bool;
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=6 */

impl<T> IsDefault for Option<T> {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=6 */

impl IsDefault for AttrFlags {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=6 */

impl IsDefault for bool {
    fn is_default(&self) -> bool {
        !self
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=6 */

impl IsDefault for u32 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=6 */

impl IsDefault for u64 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=6 */

impl<T> IsDefault for LazyArray<T> {
    fn is_default(&self) -> bool {
        self.num_elems == 0
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=is_default | COMPLEXITY=5 | LINES=10 */

impl IsDefault for UnusedGenericParams {
    fn is_default(&self) -> bool {
        // UnusedGenericParams encodes the *un*usedness as a bitset.
        // This means that 0 corresponds to all bits used, which is indeed the default.
        let is_default = self.bits() == 0;
        debug_assert_eq!(is_default, self.all_used());
        is_default
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=7 | LINES=13 */

/// Helper trait, for encoding to, and decoding from, a fixed number of bytes.
/// Used mainly for Lazy positions and lengths.
/// Unchecked invariant: `Self::default()` should encode as `[0; BYTE_LEN]`,
/// but this has no impact on safety.
pub(super) trait FixedSizeEncoding: IsDefault {
    /// This should be `[u8; BYTE_LEN]`;
    /// Cannot use an associated `const BYTE_LEN: usize` instead due to const eval limitations.
    type ByteArray;

    fn from_bytes(b: &Self::ByteArray) -> Self;
    fn write_to_bytes(self, b: &mut Self::ByteArray);
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=6 | LINES=14 */

impl FixedSizeEncoding for u64 {
    type ByteArray = [u8; 8];

    #[inline]
    fn from_bytes(b: &[u8; 8]) -> Self {
        Self::from_le_bytes(*b)
    }

    #[inline]
    fn write_to_bytes(self, b: &mut [u8; 8]) {
        *b = self.to_le_bytes();
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=31 | LINES=30 */

macro_rules! fixed_size_enum {
    ($ty:ty { $(($($pat:tt)*))* } $( unreachable { $(($($upat:tt)*))+ } )?) => {
        impl FixedSizeEncoding for Option<$ty> {
            type ByteArray = [u8;1];

            #[inline]
            fn from_bytes(b: &[u8;1]) -> Self {
                use $ty::*;
                if b[0] == 0 {
                    return None;
                }
                match b[0] - 1 {
                    $(${index()} => Some($($pat)*),)*
                    _ => panic!("Unexpected {} code: {:?}", stringify!($ty), b[0]),
                }
            }

            #[inline]
            fn write_to_bytes(self, b: &mut [u8;1]) {
                use $ty::*;
                b[0] = match self {
                    None => unreachable!(),
                    $(Some($($pat)*) => 1 + ${index()},)*
                    $(Some($($($upat)*)|+) => unreachable!(),)?
                }
            }
        }
    }
}
/* AST_META: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=7 | LINES=5 */

// Workaround; need const traits to construct bitflags in a const
macro_rules! const_macro_kinds {
    ($($name:ident),+$(,)?) => (MacroKinds::from_bits_truncate($(MacroKinds::$name.bits())|+))
}
/* AST_META: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=23 | LINES=61 */
const MACRO_KINDS_ATTR_BANG: MacroKinds = const_macro_kinds!(ATTR, BANG);
const MACRO_KINDS_DERIVE_BANG: MacroKinds = const_macro_kinds!(DERIVE, BANG);
const MACRO_KINDS_DERIVE_ATTR: MacroKinds = const_macro_kinds!(DERIVE, ATTR);
const MACRO_KINDS_DERIVE_ATTR_BANG: MacroKinds = const_macro_kinds!(DERIVE, ATTR, BANG);
// Ensure that we get a compilation error if MacroKinds gets extended without updating metadata.
const _: () = assert!(MACRO_KINDS_DERIVE_ATTR_BANG.is_all());

fixed_size_enum! {
    DefKind {
        ( Mod                                      )
        ( Struct                                   )
        ( Union                                    )
        ( Enum                                     )
        ( Variant                                  )
        ( Trait                                    )
        ( TyAlias                                  )
        ( ForeignTy                                )
        ( TraitAlias                               )
        ( AssocTy                                  )
        ( TyParam                                  )
        ( Fn                                       )
        ( Const                                    )
        ( ConstParam                               )
        ( AssocFn                                  )
        ( AssocConst                               )
        ( ExternCrate                              )
        ( Use                                      )
        ( ForeignMod                               )
        ( AnonConst                                )
        ( InlineConst                              )
        ( OpaqueTy                                 )
        ( Field                                    )
        ( LifetimeParam                            )
        ( GlobalAsm                                )
        ( Impl { of_trait: false }                 )
        ( Impl { of_trait: true }                  )
        ( Closure                                  )
        ( Static { safety: hir::Safety::Unsafe, mutability: ast::Mutability::Not, nested: false } )
        ( Static { safety: hir::Safety::Safe, mutability: ast::Mutability::Not, nested: false } )
        ( Static { safety: hir::Safety::Unsafe, mutability: ast::Mutability::Mut, nested: false } )
        ( Static { safety: hir::Safety::Safe, mutability: ast::Mutability::Mut, nested: false } )
        ( Static { safety: hir::Safety::Unsafe, mutability: ast::Mutability::Not, nested: true } )
        ( Static { safety: hir::Safety::Safe, mutability: ast::Mutability::Not, nested: true } )
        ( Static { safety: hir::Safety::Unsafe, mutability: ast::Mutability::Mut, nested: true } )
        ( Static { safety: hir::Safety::Safe, mutability: ast::Mutability::Mut, nested: true } )
        ( Ctor(CtorOf::Struct, CtorKind::Fn)       )
        ( Ctor(CtorOf::Struct, CtorKind::Const)    )
        ( Ctor(CtorOf::Variant, CtorKind::Fn)      )
        ( Ctor(CtorOf::Variant, CtorKind::Const)   )
        ( Macro(MacroKinds::BANG)                  )
        ( Macro(MacroKinds::ATTR)                  )
        ( Macro(MacroKinds::DERIVE)                )
        ( Macro(MACRO_KINDS_ATTR_BANG)             )
        ( Macro(MACRO_KINDS_DERIVE_ATTR)           )
        ( Macro(MACRO_KINDS_DERIVE_BANG)           )
        ( Macro(MACRO_KINDS_DERIVE_ATTR_BANG)      )
        ( SyntheticCoroutineBody                   )
    } unreachable {
        ( Macro(_)                                 )
    }
}
/* AST_META: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=7 */

fixed_size_enum! {
    hir::Constness {
        ( NotConst )
        ( Const    )
    }
}
/* AST_META: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=5 | LINES=8 */

fixed_size_enum! {
    hir::Defaultness {
        ( Final                        )
        ( Default { has_value: false } )
        ( Default { has_value: true }  )
    }
}
/* AST_META: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=7 */

fixed_size_enum! {
    hir::Safety {
        ( Unsafe )
        ( Safe   )
    }
}
/* AST_META: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=7 */

fixed_size_enum! {
    ty::Asyncness {
        ( Yes )
        ( No  )
    }
}
/* AST_META: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=5 | LINES=16 */

fixed_size_enum! {
    hir::CoroutineKind {
        ( Coroutine(hir::Movability::Movable)                                          )
        ( Coroutine(hir::Movability::Static)                                           )
        ( Desugared(hir::CoroutineDesugaring::Gen, hir::CoroutineSource::Block)        )
        ( Desugared(hir::CoroutineDesugaring::Gen, hir::CoroutineSource::Fn)           )
        ( Desugared(hir::CoroutineDesugaring::Gen, hir::CoroutineSource::Closure)      )
        ( Desugared(hir::CoroutineDesugaring::Async, hir::CoroutineSource::Block)      )
        ( Desugared(hir::CoroutineDesugaring::Async, hir::CoroutineSource::Fn)         )
        ( Desugared(hir::CoroutineDesugaring::Async, hir::CoroutineSource::Closure)    )
        ( Desugared(hir::CoroutineDesugaring::AsyncGen, hir::CoroutineSource::Block)   )
        ( Desugared(hir::CoroutineDesugaring::AsyncGen, hir::CoroutineSource::Fn)      )
        ( Desugared(hir::CoroutineDesugaring::AsyncGen, hir::CoroutineSource::Closure) )
    }
}
/* AST_META: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=8 */

fixed_size_enum! {
    MacroKind {
        ( Attr   )
        ( Bang   )
        ( Derive )
    }
}
/* AST_META: AST_ID=20 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=18 | LINES=34 */

// We directly encode RawDefId because using a `LazyValue` would incur a 50% overhead in the worst case.
impl FixedSizeEncoding for Option<RawDefId> {
    type ByteArray = [u8; 8];

    #[inline]
    fn from_bytes(encoded: &[u8; 8]) -> Self {
        let (index, krate) = decode_interleaved(encoded);
        let krate = u32::from_le_bytes(krate);
        if krate == 0 {
            return None;
        }
        let index = u32::from_le_bytes(index);

        Some(RawDefId { krate: krate - 1, index })
    }

    #[inline]
    fn write_to_bytes(self, dest: &mut [u8; 8]) {
        match self {
            None => unreachable!(),
            Some(RawDefId { krate, index }) => {
                debug_assert!(krate < u32::MAX);
                // CrateNum is less than `CrateNum::MAX_AS_U32`.
                let krate = (krate + 1).to_le_bytes();
                let index = index.to_le_bytes();

                // CrateNum is usually much smaller than the index within the crate, so put it in
                // the second slot.
                encode_interleaved(index, krate, dest);
            }
        }
    }
}
/* AST_META: AST_ID=21 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=6 | LINES=15 */

impl FixedSizeEncoding for AttrFlags {
    type ByteArray = [u8; 1];

    #[inline]
    fn from_bytes(b: &[u8; 1]) -> Self {
        AttrFlags::from_bits_truncate(b[0])
    }

    #[inline]
    fn write_to_bytes(self, b: &mut [u8; 1]) {
        debug_assert!(!self.is_default());
        b[0] = self.bits();
    }
}
/* AST_META: AST_ID=22 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=6 | LINES=15 */

impl FixedSizeEncoding for bool {
    type ByteArray = [u8; 1];

    #[inline]
    fn from_bytes(b: &[u8; 1]) -> Self {
        b[0] != 0
    }

    #[inline]
    fn write_to_bytes(self, b: &mut [u8; 1]) {
        debug_assert!(!self.is_default());
        b[0] = self as u8
    }
}
/* AST_META: AST_ID=23 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=14 | LINES=25 */

// NOTE(eddyb) there could be an impl for `usize`, which would enable a more
// generic `LazyValue<T>` impl, but in the general case we might not need / want
// to fit every `usize` in `u32`.
impl<T> FixedSizeEncoding for Option<LazyValue<T>> {
    type ByteArray = [u8; 8];

    #[inline]
    fn from_bytes(b: &[u8; 8]) -> Self {
        let position = NonZero::new(u64::from_bytes(b) as usize)?;
        Some(LazyValue::from_position(position))
    }

    #[inline]
    fn write_to_bytes(self, b: &mut [u8; 8]) {
        match self {
            None => unreachable!(),
            Some(lazy) => {
                let position = lazy.position.get();
                let position: u64 = position.try_into().unwrap();
                position.write_to_bytes(b)
            }
        }
    }
}
/* AST_META: AST_ID=24 | TYPE=FUNCTION | NAME=write_to_bytes_impl | COMPLEXITY=5 | LINES=16 */

impl<T> LazyArray<T> {
    #[inline]
    fn write_to_bytes_impl(self, dest: &mut [u8; 16]) {
        let position = (self.position.get() as u64).to_le_bytes();
        let len = (self.num_elems as u64).to_le_bytes();

        encode_interleaved(position, len, dest)
    }

    fn from_bytes_impl(position: &[u8; 8], meta: &[u8; 8]) -> Option<LazyArray<T>> {
        let position = NonZero::new(u64::from_bytes(position) as usize)?;
        let len = u64::from_bytes(meta) as usize;
        Some(LazyArray::from_position_and_num_elems(position, len))
    }
}
/* AST_META: AST_ID=25 | TYPE=FUNCTION | NAME=decode_interleaved | COMPLEXITY=7 | LINES=14 */

// Interleaving the bytes of the two integers exposes trailing bytes in the first integer
// to the varint scheme that we use for tables.
#[inline]
fn decode_interleaved<const N: usize, const M: usize>(encoded: &[u8; N]) -> ([u8; M], [u8; M]) {
    assert_eq!(M * 2, N);
    let mut first = [0u8; M];
    let mut second = [0u8; M];
    for i in 0..M {
        first[i] = encoded[2 * i];
        second[i] = encoded[2 * i + 1];
    }
    (first, second)
}
/* AST_META: AST_ID=26 | TYPE=FUNCTION | NAME=encode_interleaved | COMPLEXITY=6 | LINES=16 */

// Element width is selected at runtime on a per-table basis by omitting trailing
// zero bytes in table elements. This works very naturally when table elements are
// simple numbers but sometimes we have a pair of integers. If naively encoded, the second element
// would shield the trailing zeroes in the first. Interleaving the bytes exposes trailing zeroes in
// both to the optimization.
//
// Prefer passing a and b such that `b` is usually smaller.
#[inline]
fn encode_interleaved<const N: usize, const M: usize>(a: [u8; M], b: [u8; M], dest: &mut [u8; N]) {
    assert_eq!(M * 2, N);
    for i in 0..M {
        dest[2 * i] = a[i];
        dest[2 * i + 1] = b[i];
    }
}
/* AST_META: AST_ID=27 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=9 | LINES=20 */

impl<T> FixedSizeEncoding for LazyArray<T> {
    type ByteArray = [u8; 16];

    #[inline]
    fn from_bytes(b: &[u8; 16]) -> Self {
        let (position, meta) = decode_interleaved(b);

        if meta == [0; 8] {
            return Default::default();
        }
        LazyArray::from_bytes_impl(&position, &meta).unwrap()
    }

    #[inline]
    fn write_to_bytes(self, b: &mut [u8; 16]) {
        assert!(!self.is_default());
        self.write_to_bytes_impl(b)
    }
}
/* AST_META: AST_ID=28 | TYPE=FUNCTION | NAME=from_bytes | COMPLEXITY=10 | LINES=19 */

impl<T> FixedSizeEncoding for Option<LazyArray<T>> {
    type ByteArray = [u8; 16];

    #[inline]
    fn from_bytes(b: &[u8; 16]) -> Self {
        let (position, meta) = decode_interleaved(b);

        LazyArray::from_bytes_impl(&position, &meta)
    }

    #[inline]
    fn write_to_bytes(self, b: &mut [u8; 16]) {
        match self {
            None => unreachable!(),
            Some(lazy) => lazy.write_to_bytes_impl(b),
        }
    }
}
/* AST_META: AST_ID=29 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=7 */

/// Helper for constructing a table's serialization (also see `Table`).
pub(super) struct TableBuilder<I: Idx, T: FixedSizeEncoding> {
    width: usize,
    blocks: IndexVec<I, T::ByteArray>,
    _marker: PhantomData<T>,
}
/* AST_META: AST_ID=30 | TYPE=FUNCTION | NAME=default | COMPLEXITY=6 | LINES=6 */

impl<I: Idx, T: FixedSizeEncoding> Default for TableBuilder<I, T> {
    fn default() -> Self {
        TableBuilder { width: 0, blocks: Default::default(), _marker: PhantomData }
    }
}
/* AST_META: AST_ID=31 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=9 */

impl<I: Idx, const N: usize, T> TableBuilder<I, Option<T>>
where
    Option<T>: FixedSizeEncoding<ByteArray = [u8; N]>,
{
    pub(crate) fn set_some(&mut self, i: I, value: T) {
        self.set(i, Some(value))
    }
}
/* AST_META: AST_ID=32 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=23 | LINES=41 */

impl<I: Idx, const N: usize, T: FixedSizeEncoding<ByteArray = [u8; N]>> TableBuilder<I, T> {
    /// Sets the table value if it is not default.
    /// ATTENTION: For optimization default values are simply ignored by this function, because
    /// right now metadata tables never need to reset non-default values to default. If such need
    /// arises in the future then a new method (e.g. `clear` or `reset`) will need to be introduced
    /// for doing that explicitly.
    pub(crate) fn set(&mut self, i: I, value: T) {
        if !value.is_default() {
            // FIXME(eddyb) investigate more compact encodings for sparse tables.
            // On the PR @michaelwoerister mentioned:
            // > Space requirements could perhaps be optimized by using the HAMT `popcnt`
            // > trick (i.e. divide things into buckets of 32 or 64 items and then
            // > store bit-masks of which item in each bucket is actually serialized).
            let block = self.blocks.ensure_contains_elem(i, || [0; N]);
            value.write_to_bytes(block);
            if self.width != N {
                let width = N - trailing_zeros(block);
                self.width = self.width.max(width);
            }
        }
    }

    pub(crate) fn encode(&self, buf: &mut FileEncoder) -> LazyTable<I, T> {
        let pos = buf.position();

        let width = self.width;
        for block in &self.blocks {
            buf.write_with(|dest| {
                *dest = *block;
                width
            });
        }

        LazyTable::from_position_and_encoded_size(
            NonZero::new(pos).unwrap(),
            width,
            self.blocks.len(),
        )
    }
}
/* AST_META: AST_ID=33 | TYPE=FUNCTION | NAME=trailing_zeros | COMPLEXITY=2 | LINES=4 */

fn trailing_zeros(x: &[u8]) -> usize {
    x.iter().rev().take_while(|b| **b == 0).count()
}
/* AST_META: AST_ID=34 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=32 */

impl<I: Idx, const N: usize, T: FixedSizeEncoding<ByteArray = [u8; N]> + ParameterizedOverTcx>
    LazyTable<I, T>
where
    for<'tcx> T::Value<'tcx>: FixedSizeEncoding<ByteArray = [u8; N]>,
{
    /// Given the metadata, extract out the value at a particular index (if any).
    pub(super) fn get<'a, 'tcx, M: Metadata<'a, 'tcx>>(&self, metadata: M, i: I) -> T::Value<'tcx> {
        // Access past the end of the table returns a Default
        if i.index() >= self.len {
            return Default::default();
        }

        let width = self.width;
        let start = self.position.get() + (width * i.index());
        let end = start + width;
        let bytes = &metadata.blob()[start..end];

        if let Ok(fixed) = bytes.try_into() {
            FixedSizeEncoding::from_bytes(fixed)
        } else {
            let mut fixed = [0u8; N];
            fixed[..width].copy_from_slice(bytes);
            FixedSizeEncoding::from_bytes(&fixed)
        }
    }

    /// Size of the table in entries, including possible gaps.
    pub(super) fn size(&self) -> usize {
        self.len
    }
}
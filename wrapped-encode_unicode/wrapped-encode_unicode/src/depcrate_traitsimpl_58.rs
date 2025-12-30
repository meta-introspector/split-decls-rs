// Generated macro for impl_58 (impl)
macro_rules! Depcrate_traitsimpl_58 {
() => {
// Module: crate::traits
// Provides: {"impl_58"}
// Dependencies: {}
impl < S : ? Sized + Index < RangeFull > > SliceExt for S { fn utf8char_indices (& self) -> Utf8CharDecoder where Self :: Output : Borrow < [u8] > { Utf8CharDecoder :: from (self [..] . borrow ()) } fn utf16char_indices (& self) -> Utf16CharDecoder where Self :: Output : Borrow < [u16] > { Utf16CharDecoder :: from (self [..] . borrow ()) } }
};
}

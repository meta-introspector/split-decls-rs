// Generated macro for impl_lentype (macro)
macro_rules! Depcrate_len_typeimpl_lentype {
() => {
// Module: crate::len_type
// Provides: {"impl_lentype"}
// Dependencies: {}
macro_rules ! impl_lentype { ($ ($ (# [$ meta : meta]) * $ LenT : ty) ,*) => { $ ($ (# [$ meta]) * impl Sealed for $ LenT { const ZERO : Self = 0 ; const MAX : Self = Self :: MAX ; const MAX_USIZE : usize = Self :: MAX as _ ; fn one () -> Self { 1 } } $ (# [$ meta]) * impl LenType for $ LenT { }) * } }
};
}

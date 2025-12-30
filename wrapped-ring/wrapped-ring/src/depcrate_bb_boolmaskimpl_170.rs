// Generated macro for impl_170 (impl)
macro_rules! Depcrate_bb_boolmaskimpl_170 {
() => {
// Module: crate::bb::boolmask
// Provides: {"impl_170"}
// Dependencies: {}
impl BoolMask { pub (crate) const TRUE : Self = Self (Word :: MAX) ; pub (crate) const FALSE : Self = Self (0) ; # [doc = " Returns true if `self` is `BoolMask::TRUE`; otherwise, returns false"] # [doc = " (`self` is `BoolMask::FALSE`)."] pub (crate) fn leak (self) -> bool { self . 0 != 0 } }
};
}

// Generated macro for impl_37 (impl)
macro_rules! Depcrate_foreign_alloc_ffi_c_strimpl_37 {
() => {
// Module: crate::foreign::alloc::ffi::c_str
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for CString { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < Vec < u8 > as Arbitrary > :: arbitrary (u) . map (| mut x | { x . retain (| & c | c != 0) ; unsafe { Self :: from_vec_unchecked (x) } }) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < Vec < u8 > as Arbitrary > :: size_hint (depth) } }
};
}

// Generated macro for impl_156 (impl)
macro_rules! Depcrate_foreign_std_ffi_os_strimpl_156 {
() => {
// Module: crate::foreign::std::ffi::os_str
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for OsString { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < String as Arbitrary > :: arbitrary (u) . map (From :: from) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < String as Arbitrary > :: size_hint (depth) } }
};
}

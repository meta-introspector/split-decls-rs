// Generated macro for impl_167 (impl)
macro_rules! Depcrate_foreign_std_pathimpl_167 {
() => {
// Module: crate::foreign::std::path
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for PathBuf { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < OsString as Arbitrary > :: arbitrary (u) . map (From :: from) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < OsString as Arbitrary > :: size_hint (depth) } }
};
}

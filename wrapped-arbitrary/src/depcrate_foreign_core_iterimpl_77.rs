// Generated macro for impl_77 (impl)
macro_rules! Depcrate_foreign_core_iterimpl_77 {
() => {
// Module: crate::foreign::core::iter
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for Empty < A > where A : Arbitrary < 'a > , { fn arbitrary (_ : & mut Unstructured < 'a >) -> Result < Self > { Ok (empty ()) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}

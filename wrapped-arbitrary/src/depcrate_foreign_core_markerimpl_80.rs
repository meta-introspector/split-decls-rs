// Generated macro for impl_80 (impl)
macro_rules! Depcrate_foreign_core_markerimpl_80 {
() => {
// Module: crate::foreign::core::marker
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a , A > Arbitrary < 'a > for PhantomData < A > where A : ? Sized , { fn arbitrary (_ : & mut Unstructured < 'a >) -> Result < Self > { Ok (PhantomData) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , Some (0)) } }
};
}

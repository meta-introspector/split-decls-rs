// Generated macro for impl_127 (impl)
macro_rules! Depcrate_foreign_core_strimpl_127 {
() => {
// Module: crate::foreign::core::str
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for & 'a str { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { let size = u . arbitrary_len :: < u8 > () ? ; arbitrary_str (u , size) } fn arbitrary_take_rest (mut u : Unstructured < 'a >) -> Result < Self > { let size = u . len () ; arbitrary_str (& mut u , size) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (0 , None) } }
};
}

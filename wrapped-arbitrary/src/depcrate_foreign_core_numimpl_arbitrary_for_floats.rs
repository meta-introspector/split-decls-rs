// Generated macro for impl_arbitrary_for_floats (macro)
macro_rules! Depcrate_foreign_core_numimpl_arbitrary_for_floats {
() => {
// Module: crate::foreign::core::num
// Provides: {"impl_arbitrary_for_floats"}
// Dependencies: {}
macro_rules ! impl_arbitrary_for_floats { ($ ($ ty : ident : $ unsigned : ty ;) *) => { $ (impl <'a > Arbitrary <'a > for $ ty { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { Ok (Self :: from_bits (<$ unsigned as Arbitrary <'a >>:: arbitrary (u) ?)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { <$ unsigned as Arbitrary <'a >>:: size_hint (depth) } }) * } }
};
}

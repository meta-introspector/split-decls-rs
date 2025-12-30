// Generated macro for impl_arbitrary_for_integers (macro)
macro_rules! Depcrate_foreign_core_numimpl_arbitrary_for_integers {
() => {
// Module: crate::foreign::core::num
// Provides: {"impl_arbitrary_for_integers"}
// Dependencies: {}
macro_rules ! impl_arbitrary_for_integers { ($ ($ ty : ty ;) *) => { $ (impl <'a > Arbitrary <'a > for $ ty { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { let mut buf = [0 ; mem :: size_of ::<$ ty > ()] ; u . fill_buffer (& mut buf) ?; Ok (Self :: from_le_bytes (buf)) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { let n = mem :: size_of ::<$ ty > () ; (n , Some (n)) } }) * } }
};
}

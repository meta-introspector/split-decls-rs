// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : Neg > Neg for OrderedFloat < T > { type Output = OrderedFloat < T :: Output > ; # [inline] fn neg (self) -> Self :: Output { OrderedFloat (- self . 0) } }
};
}

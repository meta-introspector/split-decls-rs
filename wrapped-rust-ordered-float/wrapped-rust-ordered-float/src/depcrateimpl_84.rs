// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , T > Neg for & 'a OrderedFloat < T > where & 'a T : Neg , { type Output = OrderedFloat < < & 'a T as Neg > :: Output > ; # [inline] fn neg (self) -> Self :: Output { OrderedFloat (- (& self . 0)) } }
};
}

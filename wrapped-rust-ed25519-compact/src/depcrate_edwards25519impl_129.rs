// Generated macro for impl_129 (impl)
macro_rules! Depcrate_edwards25519impl_129 {
() => {
// Module: crate::edwards25519
// Provides: {"impl_129"}
// Dependencies: {}
impl GeP1P1 { fn to_p2 (& self) -> GeP2 { GeP2 { x : self . x * self . t , y : self . y * self . z , z : self . z * self . t , } } fn to_p3 (& self) -> GeP3 { GeP3 { x : self . x * self . t , y : self . y * self . z , z : self . z * self . t , t : self . x * self . y , } } }
};
}

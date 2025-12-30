// Generated macro for impl_19 (impl)
macro_rules! Depcrate_castimpl_19 {
() => {
// Module: crate::cast
// Provides: {"impl_19"}
// Dependencies: {}
impl < T , U > AsPrimitive < U > for Complex < T > where T : AsPrimitive < U > , U : 'static + Copy , { fn as_ (self) -> U { self . re . as_ () } }
};
}

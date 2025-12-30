// Generated macro for impl_328 (impl)
macro_rules! Depcrate_valueimpl_328 {
() => {
// Module: crate::value
// Provides: {"impl_328"}
// Dependencies: {}
impl < T > From < T > for Value where T : Into < ValueKind > , { fn from (value : T) -> Self { Self { origin : None , kind : value . into () , } } }
};
}

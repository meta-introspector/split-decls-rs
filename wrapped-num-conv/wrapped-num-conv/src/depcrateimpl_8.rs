// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : sealed :: Integer > Truncate for T { fn truncate < U > (self) -> U where T : TruncateTarget < U > , { sealed :: TruncateTargetSealed :: truncate (self) } }
};
}

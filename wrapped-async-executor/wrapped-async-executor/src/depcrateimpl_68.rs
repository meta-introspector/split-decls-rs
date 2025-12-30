// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl < Fut , Cleanup : FnMut () > AsyncCallOnDrop < Fut , Cleanup > { fn new (future : Fut , cleanup : Cleanup) -> Self { Self { future , cleanup : CallOnDrop (cleanup) , } } }
};
}

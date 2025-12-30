// Generated macro for impl_17 (impl)
macro_rules! Depcrate_versionsimpl_17 {
() => {
// Module: crate::versions
// Provides: {"impl_17"}
// Dependencies: {}
impl From < Versions > for State { fn from (versions : Versions) -> Self { match versions { Versions :: Legacy (state) => * state , Versions :: Current (state) => * state , } } }
};
}

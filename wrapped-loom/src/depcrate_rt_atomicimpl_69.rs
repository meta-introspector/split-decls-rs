// Generated macro for impl_69 (impl)
macro_rules! Depcrate_rt_atomicimpl_69 {
() => {
// Module: crate::rt::atomic
// Provides: {"impl_69"}
// Dependencies: {}
impl Default for Store { fn default () -> Store { Store { value : 0 , happens_before : VersionVec :: new () , modification_order : VersionVec :: new () , sync : Synchronize :: new () , first_seen : FirstSeen :: new () , seq_cst : false , } } }
};
}

// Generated macro for test (module)
macro_rules! Depcrate_tokiotest {
() => {
// Module: crate::tokio
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { async fn compute () -> u8 { tokio :: spawn (async { 1 + 2 }) . await . unwrap () } # [test] fn spawn_tokio () { crate :: block_on (async { assert_eq ! (crate :: spawn (compute ()) . await + crate :: spawn_local (compute ()) . await + tokio :: spawn (compute ()) . await . unwrap () , 9) ; }) ; } }
};
}

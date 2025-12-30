// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_test_runner_runnerimpl_1264 {
() => {
// Module: crate::test_runner::runner
// Provides: {"impl_1264"}
// Dependencies: {}
# [cfg (feature = "fork")] impl ForkOutput { fn append (& mut self , result : & TestCaseResult) { if let Some (ref mut file) = self . file { replay :: append (file , result) . expect ("Failed to append to replay file") ; } } fn ping (& mut self) { if let Some (ref mut file) = self . file { replay :: ping (file) . expect ("Failed to append to replay file") ; } } fn terminate (& mut self) { if let Some (ref mut file) = self . file { replay :: terminate (file) . expect ("Failed to append to replay file") ; } } fn empty () -> Self { ForkOutput { file : None } } fn is_in_fork (& self) -> bool { self . file . is_some () } }
};
}

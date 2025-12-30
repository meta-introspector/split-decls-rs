// Generated macro for impl_667 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_667 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_667"}
// Dependencies: {}
impl Step for RemoteCopyLibs { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) { let build_compiler = self . build_compiler ; let target = self . target ; if ! builder . remote_tested (target) { return ; } builder . std (build_compiler , target) ; builder . info (& format ! ("REMOTE copy libs to emulator ({target})")) ; let remote_test_server = builder . ensure (tool :: RemoteTestServer { build_compiler , target }) ; let tool = builder . tool_exe (Tool :: RemoteTestClient) ; let mut cmd = command (& tool) ; cmd . arg ("spawn-emulator") . arg (target . triple) . arg (& remote_test_server . tool_path) . arg (builder . tempdir ()) ; if let Some (rootfs) = builder . qemu_rootfs (target) { cmd . arg (rootfs) ; } cmd . run (builder) ; for f in t ! (builder . sysroot_target_libdir (build_compiler , target) . read_dir ()) { let f = t ! (f) ; if helpers :: is_dylib (& f . path ()) { command (& tool) . arg ("push") . arg (f . path ()) . run (builder) ; } } } }
};
}

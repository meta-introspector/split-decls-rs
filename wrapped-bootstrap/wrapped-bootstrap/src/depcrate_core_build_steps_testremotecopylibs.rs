// Generated macro for RemoteCopyLibs (struct)
macro_rules! Depcrate_core_build_steps_testRemoteCopyLibs {
() => {
// Module: crate::core::build_steps::test
// Provides: {"RemoteCopyLibs"}
// Dependencies: {}
# [doc = " Some test suites are run inside emulators or on remote devices, and most"] # [doc = " of our test binaries are linked dynamically which means we need to ship"] # [doc = " the standard library and such to the emulator ahead of time. This step"] # [doc = " represents this and is a dependency of all test suites."] # [doc = ""] # [doc = " Most of the time this is a no-op. For some steps such as shipping data to"] # [doc = " QEMU we have to build our own tools so we've got conditional dependencies"] # [doc = " on those programs as well. Note that the remote test client is built for"] # [doc = " the build target (us) and the server is built for the target."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct RemoteCopyLibs { build_compiler : Compiler , target : TargetSelection , }
};
}

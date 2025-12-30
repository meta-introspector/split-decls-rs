// Generated macro for Miri (struct)
macro_rules! Depcrate_core_build_steps_runMiri {
() => {
// Module: crate::core::build_steps::run
// Provides: {"Miri"}
// Dependencies: {}
# [doc = " Invoke the Miri tool on a specified file."] # [doc = ""] # [doc = " Note that Miri always executed on the host, as it is an interpreter."] # [doc = " That means that `x run miri --target FOO` will build miri for the host,"] # [doc = " prepare a miri sysroot for the target `FOO` and then execute miri with"] # [doc = " the target `FOO`."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Miri { # [doc = " The build compiler that will build miri and the target compiler to which miri links."] compilers : RustcPrivateCompilers , # [doc = " The target which will miri interpret."] target : TargetSelection , }
};
}

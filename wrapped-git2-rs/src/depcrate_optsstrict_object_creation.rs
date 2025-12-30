// Generated macro for strict_object_creation (function)
macro_rules! Depcrate_optsstrict_object_creation {
() => {
// Module: crate::opts
// Provides: {"strict_object_creation"}
// Dependencies: {}
# [doc = " Controls whether or not libgit2 will verify when writing an object that all"] # [doc = " objects it references are valid. Enabled by default, but disabling this can"] # [doc = " significantly improve performance, at the cost of potentially allowing the"] # [doc = " creation of objects that reference invalid objects (due to programming"] # [doc = " error or repository corruption)."] pub fn strict_object_creation (enabled : bool) { crate :: init () ; let error = unsafe { raw :: git_libgit2_opts (raw :: GIT_OPT_ENABLE_STRICT_OBJECT_CREATION as libc :: c_int , enabled as libc :: c_int ,) } ; debug_assert ! (error >= 0) ; }
};
}

// Generated macro for strict_hash_verification (function)
macro_rules! Depcrate_optsstrict_hash_verification {
() => {
// Module: crate::opts
// Provides: {"strict_hash_verification"}
// Dependencies: {}
# [doc = " Controls whether or not libgit2 will verify that objects loaded have the"] # [doc = " expected hash. Enabled by default, but disabling this can significantly"] # [doc = " improve performance, at the cost of relying on repository integrity"] # [doc = " without checking it."] pub fn strict_hash_verification (enabled : bool) { crate :: init () ; let error = unsafe { raw :: git_libgit2_opts (raw :: GIT_OPT_ENABLE_STRICT_HASH_VERIFICATION as libc :: c_int , enabled as libc :: c_int ,) } ; debug_assert ! (error >= 0) ; }
};
}

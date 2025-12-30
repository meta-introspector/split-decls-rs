// Generated macro for build_notices (function)
macro_rules! Depcrate_arm_configbuild_notices {
() => {
// Module: crate::arm::config
// Provides: {"build_notices"}
// Dependencies: {}
pub fn build_notices (line_prefix : & str) -> String { format ! ("\
{line_prefix}This is a transient test file, not intended for distribution. Some aspects of the
{line_prefix}test are derived from a JSON specification, published under the same license as the
{line_prefix}`intrinsic-test` crate.\n
") }
};
}

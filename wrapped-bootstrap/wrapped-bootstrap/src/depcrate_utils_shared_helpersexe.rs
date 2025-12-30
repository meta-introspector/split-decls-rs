// Generated macro for exe (function)
macro_rules! Depcrate_utils_shared_helpersexe {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"exe"}
// Dependencies: {}
# [doc = " Given an executable called `name`, return the filename for the"] # [doc = " executable for a particular target."] pub fn exe (name : & str , target : & str) -> String { if target . contains ("windows") || (cfg ! (not (target_os = "cygwin")) && target . contains ("cygwin")) { format ! ("{name}.exe") } else if target . contains ("uefi") { format ! ("{name}.efi") } else if target . contains ("wasm") { format ! ("{name}.wasm") } else { name . to_string () } }
};
}

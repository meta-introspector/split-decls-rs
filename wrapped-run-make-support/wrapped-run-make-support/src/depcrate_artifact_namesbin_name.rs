// Generated macro for bin_name (function)
macro_rules! Depcrate_artifact_namesbin_name {
() => {
// Module: crate::artifact_names
// Provides: {"bin_name"}
// Dependencies: {}
# [doc = " Construct the binary (executable) name based on the target."] # [track_caller] # [must_use] pub fn bin_name (name : & str) -> String { let target = target () ; if target . contains ("windows") { format ! ("{name}.exe") } else if target . contains ("uefi") { format ! ("{name}.efi") } else if target . contains ("wasm") { format ! ("{name}.wasm") } else if target . contains ("nvptx") { format ! ("{name}.ptx") } else { name . to_string () } }
};
}

// Generated macro for copy_recursive (function)
macro_rules! Depcrate_utilcopy_recursive {
() => {
// Module: crate::util
// Provides: {"copy_recursive"}
// Dependencies: {}
# [doc = " Copies the `src` directory recursively to `dst`. Both are assumed to exist"] # [doc = " when this function is called."] pub fn copy_recursive (src : & Path , dst : & Path) -> Result < () > { copy_with_callback (src , dst , | _ , _ | Ok (())) }
};
}

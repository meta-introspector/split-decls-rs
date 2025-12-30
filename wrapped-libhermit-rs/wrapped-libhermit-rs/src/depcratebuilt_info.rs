// Generated macro for built_info (module)
macro_rules! Depcratebuilt_info {
() => {
// Module: crate
// Provides: {"built_info"}
// Dependencies: {}
mod built_info { include ! (concat ! (env ! ("OUT_DIR") , "/built.rs")) ; }
};
}

// Generated macro for target_dir (function)
macro_rules! Depcratetarget_dir {
() => {
// Module: crate
// Provides: {"target_dir"}
// Dependencies: {}
# [must_use] pub fn target_dir () -> String { env :: var ("CARGO_TARGET_DIR") . unwrap_or_else (| _ | "target" . to_owned ()) }
};
}

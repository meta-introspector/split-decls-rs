// Generated macro for toolstate_repo (function)
macro_rules! Depcrate_core_build_steps_toolstatetoolstate_repo {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"toolstate_repo"}
// Dependencies: {}
fn toolstate_repo () -> String { env :: var ("TOOLSTATE_REPO") . unwrap_or_else (| _ | "https://github.com/rust-lang-nursery/rust-toolstate.git" . to_string ()) }
};
}

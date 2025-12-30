// Generated macro for stage_dir_exists (function)
macro_rules! Depcrate_core_build_steps_setupstage_dir_exists {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"stage_dir_exists"}
// Dependencies: {}
fn stage_dir_exists (stage_path : & str) -> bool { match fs :: create_dir (stage_path) { Ok (_) => true , Err (_) => Path :: new (& stage_path) . exists () , } }
};
}

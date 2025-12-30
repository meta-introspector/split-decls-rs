// Generated macro for build_raytracer (function)
macro_rules! Depcrate_preparebuild_raytracer {
() => {
// Module: crate::prepare
// Provides: {"build_raytracer"}
// Dependencies: {}
fn build_raytracer (repo_dir : & Path) -> Result < () , String > { run_command (& [& "cargo" , & "build"] , Some (repo_dir)) ? ; let mv_target = repo_dir . join ("raytracer_cg_llvm") ; if mv_target . is_file () { remove_file (& mv_target) ? ; } run_command (& [& "mv" , & "target/debug/main" , & "raytracer_cg_llvm"] , Some (repo_dir)) ? ; Ok (()) }
};
}

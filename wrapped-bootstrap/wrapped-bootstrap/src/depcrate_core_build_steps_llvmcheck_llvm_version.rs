// Generated macro for check_llvm_version (function)
macro_rules! Depcrate_core_build_steps_llvmcheck_llvm_version {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"check_llvm_version"}
// Dependencies: {}
fn check_llvm_version (builder : & Builder < '_ > , llvm_config : & Path) { if builder . config . dry_run () { return ; } let version = get_llvm_version (builder , llvm_config) ; let mut parts = version . split ('.') . take (2) . filter_map (| s | s . parse :: < u32 > () . ok ()) ; if let (Some (major) , Some (_minor)) = (parts . next () , parts . next ()) && major >= 19 { return ; } panic ! ("\n\nbad LLVM version: {version}, need >=19\n\n") }
};
}

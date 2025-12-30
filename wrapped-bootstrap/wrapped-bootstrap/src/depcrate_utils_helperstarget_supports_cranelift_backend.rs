// Generated macro for target_supports_cranelift_backend (function)
macro_rules! Depcrate_utils_helperstarget_supports_cranelift_backend {
() => {
// Module: crate::utils::helpers
// Provides: {"target_supports_cranelift_backend"}
// Dependencies: {}
pub fn target_supports_cranelift_backend (target : TargetSelection) -> bool { if target . contains ("linux") { target . contains ("x86_64") || target . contains ("aarch64") || target . contains ("s390x") || target . contains ("riscv64gc") } else if target . contains ("darwin") { target . contains ("x86_64") || target . contains ("aarch64") } else if target . is_windows () { target . contains ("x86_64") } else { false } }
};
}

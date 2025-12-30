// Generated macro for llvm_components_contain (function)
macro_rules! Depcrate_targetsllvm_components_contain {
() => {
// Module: crate::targets
// Provides: {"llvm_components_contain"}
// Dependencies: {}
# [doc = " Check if `component` is within `LLVM_COMPONENTS`"] # [must_use] pub fn llvm_components_contain (component : & str) -> bool { env_var ("LLVM_COMPONENTS") . split_whitespace () . find (| s | s == & component) . is_some () }
};
}

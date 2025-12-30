// Generated macro for impl_21 (impl)
macro_rules! Depcrate_astimpl_21 {
() => {
// Module: crate::ast
// Provides: {"impl_21"}
// Dependencies: {}
impl Program { # [doc = " Name of the link function for a specific linked module"] pub fn link_function_name (& self , idx : usize) -> String { let hash = match & self . linked_modules [idx] { ImportModule :: Inline (idx) => ShortHash ((1 , & self . inline_js [* idx])) . to_string () , other => ShortHash ((0 , other)) . to_string () , } ; format ! ("__wbindgen_link_{}" , hash) } }
};
}

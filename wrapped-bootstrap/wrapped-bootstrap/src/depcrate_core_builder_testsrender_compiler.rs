// Generated macro for render_compiler (function)
macro_rules! Depcrate_core_builder_testsrender_compiler {
() => {
// Module: crate::core::builder::tests
// Provides: {"render_compiler"}
// Dependencies: {}
fn render_compiler (compiler : Compiler , config : & RenderConfig) -> String { format ! ("rustc {} <{}>" , compiler . stage , normalize_target (compiler . host , config)) }
};
}

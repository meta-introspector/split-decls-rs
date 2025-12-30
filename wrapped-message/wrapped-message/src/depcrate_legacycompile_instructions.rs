// Generated macro for compile_instructions (function)
macro_rules! Depcrate_legacycompile_instructions {
() => {
// Module: crate::legacy
// Provides: {"compile_instructions"}
// Dependencies: {}
fn compile_instructions (ixs : & [Instruction] , keys : & [Address]) -> Vec < CompiledInstruction > { ixs . iter () . map (| ix | compile_instruction (ix , keys)) . collect () }
};
}

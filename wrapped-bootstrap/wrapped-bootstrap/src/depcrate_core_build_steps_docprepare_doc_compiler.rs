// Generated macro for prepare_doc_compiler (function)
macro_rules! Depcrate_core_build_steps_docprepare_doc_compiler {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"prepare_doc_compiler"}
// Dependencies: {}
# [doc = " Prepare a compiler that will be able to document something for `target` at `stage`."] pub fn prepare_doc_compiler (builder : & Builder < '_ > , target : TargetSelection , stage : u32 ,) -> Compiler { assert ! (stage > 0 , "Cannot document anything in stage 0") ; let build_compiler = builder . compiler (stage - 1 , builder . host_target) ; builder . std (build_compiler , target) ; build_compiler }
};
}

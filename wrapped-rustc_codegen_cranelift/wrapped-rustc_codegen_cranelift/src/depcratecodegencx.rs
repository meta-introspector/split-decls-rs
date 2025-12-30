// Generated macro for CodegenCx (struct)
macro_rules! DepcrateCodegenCx {
() => {
// Module: crate
// Provides: {"CodegenCx"}
// Dependencies: {}
# [doc = " The codegen context holds any information shared between the codegen of individual functions"] # [doc = " inside a single codegen unit with the exception of the Cranelift [`Module`](cranelift_module::Module)."] struct CodegenCx { output_filenames : Arc < OutputFilenames > , invocation_temp : Option < String > , should_write_ir : bool , global_asm : String , inline_asm_index : usize , debug_context : Option < DebugContext > , cgu_name : Symbol , }
};
}

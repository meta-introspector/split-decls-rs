// Generated macro for emit_cgu (function)
macro_rules! Depcrate_driver_aotemit_cgu {
() => {
// Module: crate::driver::aot
// Provides: {"emit_cgu"}
// Dependencies: {}
fn emit_cgu (output_filenames : & OutputFilenames , invocation_temp : Option < & str > , prof : & SelfProfilerRef , name : String , module : UnwindModule < ObjectModule > , debug : Option < DebugContext > , global_asm_object_file : Option < PathBuf > , producer : & str ,) -> Result < ModuleCodegenResult , String > { let mut product = module . finish () ; if let Some (mut debug) = debug { debug . emit (& mut product) ; } let module_regular = emit_module (output_filenames , invocation_temp , prof , product . object , ModuleKind :: Regular , name . clone () , producer ,) ? ; Ok (ModuleCodegenResult { module_regular , module_global_asm : global_asm_object_file . map (| global_asm_object_file | CompiledModule { name : format ! ("{name}.asm") , kind : ModuleKind :: Regular , object : Some (global_asm_object_file) , dwarf_object : None , bytecode : None , assembly : None , llvm_ir : None , links_from_incr_cache : Vec :: new () , }) , existing_work_product : None , }) }
};
}

// Generated macro for submit_pre_lto_module_to_llvm (function)
macro_rules! Depcrate_back_writesubmit_pre_lto_module_to_llvm {
() => {
// Module: crate::back::write
// Provides: {"submit_pre_lto_module_to_llvm"}
// Dependencies: {}
pub (crate) fn submit_pre_lto_module_to_llvm < B : ExtraBackendMethods > (tcx : TyCtxt < '_ > , coordinator : & Coordinator < B > , module : CachedModuleCodegen ,) { let filename = pre_lto_bitcode_filename (& module . name) ; let bc_path = in_incr_comp_dir_sess (tcx . sess , & filename) ; let file = fs :: File :: open (& bc_path) . unwrap_or_else (| e | panic ! ("failed to open bitcode file `{}`: {}" , bc_path . display () , e)) ; let mmap = unsafe { Mmap :: map (file) . unwrap_or_else (| e | { panic ! ("failed to mmap bitcode file `{}`: {}" , bc_path . display () , e) }) } ; drop (coordinator . sender . send (Message :: AddImportOnlyModule :: < B > { module_data : SerializedModule :: FromUncompressedFile (mmap) , work_product : module . source , })) ; }
};
}

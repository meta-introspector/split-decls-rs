// Generated macro for generate_thin_lto_work (function)
macro_rules! Depcrate_back_writegenerate_thin_lto_work {
() => {
// Module: crate::back::write
// Provides: {"generate_thin_lto_work"}
// Dependencies: {}
fn generate_thin_lto_work < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , needs_thin_lto : Vec < (String , B :: ThinBuffer) > , import_only_modules : Vec < (SerializedModule < B :: ModuleBuffer > , WorkProduct) > ,) -> Vec < (WorkItem < B > , u64) > { let _prof_timer = cgcx . prof . generic_activity ("codegen_thin_generate_lto_work") ; let (lto_modules , copy_jobs) = B :: run_thin_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , needs_thin_lto , import_only_modules ,) ; lto_modules . into_iter () . map (| module | { let cost = module . cost () ; (WorkItem :: ThinLto (module) , cost) }) . chain (copy_jobs . into_iter () . map (| wp | { (WorkItem :: CopyPostLtoArtifacts (CachedModuleCodegen { name : wp . cgu_name . clone () , source : wp , }) , 0 ,) })) . collect () }
};
}

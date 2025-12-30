// Generated macro for execute_thin_lto_work_item (function)
macro_rules! Depcrate_back_writeexecute_thin_lto_work_item {
() => {
// Module: crate::back::write
// Provides: {"execute_thin_lto_work_item"}
// Dependencies: {}
fn execute_thin_lto_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , module : lto :: ThinModule < B > ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_module_perform_lto" , module . name ()) ; let module = B :: optimize_thin (cgcx , module) ; let module = B :: codegen (cgcx , module , & cgcx . module_config) ; WorkItemResult :: Finished (module) }
};
}

macro_rules! deps {
    () => {
        ThinModule!();
        CodegenContext!();
        WorkItemResult!();
        ExtraBackendMethods!();
    };
}

macro_rules! execute_thin_lto_work_item {
    () => {
        deps!();
        fn execute_thin_lto_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , module : lto :: ThinModule < B > ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_module_perform_lto" , module . name ()) ; let module = B :: optimize_thin (cgcx , module) ; let module = B :: codegen (cgcx , module , & cgcx . module_config) ; WorkItemResult :: Finished (module) }
    };
}

execute_thin_lto_work_item!()
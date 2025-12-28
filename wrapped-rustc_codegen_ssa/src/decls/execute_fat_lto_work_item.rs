macro_rules! deps {
    () => {
        FatLtoInput!();
        SerializedModule!();
        ExtraBackendMethods!();
        CodegenContext!();
        WorkItemResult!();
    };
}

macro_rules! execute_fat_lto_work_item {
    () => {
        deps!();
        fn execute_fat_lto_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , exported_symbols_for_lto : & [String] , each_linked_rlib_for_lto : & [PathBuf] , mut needs_fat_lto : Vec < FatLtoInput < B > > , import_only_modules : Vec < (SerializedModule < B :: ModuleBuffer > , WorkProduct) > ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_module_perform_lto" , "everything") ; for (module , wp) in import_only_modules { needs_fat_lto . push (FatLtoInput :: Serialized { name : wp . cgu_name , buffer : module }) } let module = B :: run_and_optimize_fat_lto (cgcx , exported_symbols_for_lto , each_linked_rlib_for_lto , needs_fat_lto ,) ; let module = B :: codegen (cgcx , module , & cgcx . module_config) ; WorkItemResult :: Finished (module) }
    };
}

execute_fat_lto_work_item!();
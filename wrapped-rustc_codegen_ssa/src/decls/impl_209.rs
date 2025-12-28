macro_rules! deps {
    () => {
        WriteBackendMethods!();
        TargetMachineFactoryConfig!();
        CodegenContext!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl TargetMachineFactoryConfig { pub fn new (cgcx : & CodegenContext < impl WriteBackendMethods > , module_name : & str ,) -> TargetMachineFactoryConfig { let split_dwarf_file = if cgcx . target_can_use_split_dwarf { cgcx . output_filenames . split_dwarf_path (cgcx . split_debuginfo , cgcx . split_dwarf_kind , module_name , cgcx . invocation_temp . as_deref () ,) } else { None } ; let output_obj_file = Some (cgcx . output_filenames . temp_path_for_cgu (OutputType :: Object , module_name , cgcx . invocation_temp . as_deref () ,)) ; TargetMachineFactoryConfig { split_dwarf_file , output_obj_file } } }
    };
}

impl_209!();
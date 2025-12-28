macro_rules! deps {
    () => {
        OwnedTargetMachine!();
    };
}

macro_rules! create_target_machine {
    () => {
        deps!();
        pub (crate) fn create_target_machine (tcx : TyCtxt < '_ > , mod_name : & str) -> OwnedTargetMachine { let split_dwarf_file = if tcx . sess . target_can_use_split_dwarf () { tcx . output_filenames (()) . split_dwarf_path (tcx . sess . split_debuginfo () , tcx . sess . opts . unstable_opts . split_dwarf_kind , mod_name , tcx . sess . invocation_temp . as_deref () ,) } else { None } ; let output_obj_file = Some (tcx . output_filenames (()) . temp_path_for_cgu (OutputType :: Object , mod_name , tcx . sess . invocation_temp . as_deref () ,)) ; let config = TargetMachineFactoryConfig { split_dwarf_file , output_obj_file } ; target_machine_factory (tcx . sess , tcx . backend_optimization_level (()) , tcx . global_backend_features (()) ,) (config) . unwrap_or_else (| err | llvm_err (tcx . dcx () , err)) }
    };
}

create_target_machine!();
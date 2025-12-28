macro_rules! deps {
    () => {
        ModuleLlvm!();
        OwnedTargetMachine!();
        ParseTargetMachineConfig!();
        LlvmCodegenBackend!();
    };
}

macro_rules! impl_629 {
    () => {
        deps!();
        impl ModuleLlvm { fn new (tcx : TyCtxt < '_ > , mod_name : & str) -> Self { unsafe { let llcx = llvm :: LLVMRustContextCreate (tcx . sess . fewer_names ()) ; let llmod_raw = context :: create_module (tcx , llcx , mod_name) as * const _ ; ModuleLlvm { llmod_raw , llcx , tm : ManuallyDrop :: new (create_target_machine (tcx , mod_name)) , } } } fn new_metadata (tcx : TyCtxt < '_ > , mod_name : & str) -> Self { unsafe { let llcx = llvm :: LLVMRustContextCreate (tcx . sess . fewer_names ()) ; let llmod_raw = context :: create_module (tcx , llcx , mod_name) as * const _ ; ModuleLlvm { llmod_raw , llcx , tm : ManuallyDrop :: new (create_informational_target_machine (tcx . sess , false)) , } } } fn tm_from_cgcx (cgcx : & CodegenContext < LlvmCodegenBackend > , name : & str , dcx : DiagCtxtHandle < '_ > ,) -> OwnedTargetMachine { let tm_factory_config = TargetMachineFactoryConfig :: new (cgcx , name) ; match (cgcx . tm_factory) (tm_factory_config) { Ok (m) => m , Err (e) => { dcx . emit_fatal (ParseTargetMachineConfig (e)) ; } } } fn parse (cgcx : & CodegenContext < LlvmCodegenBackend > , name : & CStr , buffer : & [u8] , dcx : DiagCtxtHandle < '_ > ,) -> Self { unsafe { let llcx = llvm :: LLVMRustContextCreate (cgcx . fewer_names) ; let llmod_raw = back :: lto :: parse_module (llcx , name , buffer , dcx) ; let tm = ModuleLlvm :: tm_from_cgcx (cgcx , name . to_str () . unwrap () , dcx) ; ModuleLlvm { llmod_raw , llcx , tm : ManuallyDrop :: new (tm) } } } fn llmod (& self) -> & llvm :: Module { unsafe { & * self . llmod_raw } } }
    };
}

impl_629!();
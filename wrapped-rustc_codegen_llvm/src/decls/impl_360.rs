macro_rules! deps {
    () => {
        CodegenUnitDebugContext!();
        ModuleFlagMergeBehavior!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < 'll , 'tcx > CodegenUnitDebugContext < 'll , 'tcx > { pub (crate) fn new (llmod : & 'll llvm :: Module) -> Self { debug ! ("CodegenUnitDebugContext::new") ; let builder = DIBuilderBox :: new (llmod) ; CodegenUnitDebugContext { llmod , builder , created_files : Default :: default () , type_map : Default :: default () , adt_stack : Default :: default () , namespace_map : RefCell :: new (Default :: default ()) , recursion_marker_type : OnceCell :: new () , } } pub (crate) fn finalize (& self , sess : & Session) { unsafe { llvm :: LLVMDIBuilderFinalize (self . builder . as_ref ()) } ; match sess . target . debuginfo_kind { DebuginfoKind :: Dwarf | DebuginfoKind :: DwarfDsym => { llvm :: add_module_flag_u32 (self . llmod , llvm :: ModuleFlagMergeBehavior :: Max , "Dwarf Version" , sess . dwarf_version () ,) ; } DebuginfoKind :: Pdb => { llvm :: add_module_flag_u32 (self . llmod , llvm :: ModuleFlagMergeBehavior :: Warning , "CodeView" , 1 ,) ; } } llvm :: add_module_flag_u32 (self . llmod , llvm :: ModuleFlagMergeBehavior :: Warning , "Debug Info Version" , unsafe { llvm :: LLVMRustDebugMetadataVersion () } ,) ; } }
    };
}

impl_360!()
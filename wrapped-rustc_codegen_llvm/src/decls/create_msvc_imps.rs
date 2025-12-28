macro_rules! deps {
    () => {
        Linkage!();
        LlvmCodegenBackend!();
    };
}

macro_rules! create_msvc_imps {
    () => {
        deps!();
        fn create_msvc_imps (cgcx : & CodegenContext < LlvmCodegenBackend > , llcx : & llvm :: Context , llmod : & llvm :: Module ,) { if ! cgcx . msvc_imps_needed { return ; } let prefix = if cgcx . target_arch == "x86" { "\x01__imp__" } else { "\x01__imp_" } ; let ptr_ty = Type :: ptr_llcx (llcx) ; let globals = base :: iter_globals (llmod) . filter (| & val | { llvm :: get_linkage (val) == llvm :: Linkage :: ExternalLinkage && ! llvm :: is_declaration (val) }) . filter_map (| val | { let name = llvm :: get_value_name (val) ; if ignored (& name) { None } else { Some ((val , name)) } }) . map (move | (val , name) | { let mut imp_name = prefix . as_bytes () . to_vec () ; imp_name . extend (name) ; let imp_name = CString :: new (imp_name) . unwrap () ; (imp_name , val) }) . collect :: < Vec < _ > > () ; for (imp_name , val) in globals { let imp = llvm :: add_global (llmod , ptr_ty , & imp_name) ; llvm :: set_initializer (imp , val) ; llvm :: set_linkage (imp , llvm :: Linkage :: ExternalLinkage) ; } fn ignored (symbol_name : & [u8]) -> bool { symbol_name . starts_with (b"__llvm_profile_") } }
    };
}

create_msvc_imps!();
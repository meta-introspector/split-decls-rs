macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
        Linkage!();
    };
}

macro_rules! embed_bitcode {
    () => {
        deps!();
        # [doc = " Embed the bitcode of an LLVM module for LTO in the LLVM module itself."] fn embed_bitcode (cgcx : & CodegenContext < LlvmCodegenBackend > , llcx : & llvm :: Context , llmod : & llvm :: Module , bitcode : & [u8] ,) { if cgcx . target_is_like_darwin || cgcx . target_is_like_aix || cgcx . target_arch == "wasm32" || cgcx . target_arch == "wasm64" { let llconst = common :: bytes_in_context (llcx , bitcode) ; let llglobal = llvm :: add_global (llmod , common :: val_ty (llconst) , c"rustc.embedded.module") ; llvm :: set_initializer (llglobal , llconst) ; llvm :: set_section (llglobal , bitcode_section_name (cgcx)) ; llvm :: set_linkage (llglobal , llvm :: Linkage :: PrivateLinkage) ; llvm :: LLVMSetGlobalConstant (llglobal , llvm :: TRUE) ; let llconst = common :: bytes_in_context (llcx , & []) ; let llglobal = llvm :: add_global (llmod , common :: val_ty (llconst) , c"rustc.embedded.cmdline") ; llvm :: set_initializer (llglobal , llconst) ; let section = if cgcx . target_is_like_darwin { c"__LLVM,__cmdline" } else if cgcx . target_is_like_aix { c".info" } else { c".llvmcmd" } ; llvm :: set_section (llglobal , section) ; llvm :: set_linkage (llglobal , llvm :: Linkage :: PrivateLinkage) ; } else { let section_flags = if cgcx . is_pe_coff { "n" } else { "e" } ; let asm = create_section_with_flags_asm (".llvmbc" , section_flags , bitcode) ; llvm :: append_module_inline_asm (llmod , & asm) ; let asm = create_section_with_flags_asm (".llvmcmd" , section_flags , & []) ; llvm :: append_module_inline_asm (llmod , & asm) ; } }
    };
}

embed_bitcode!()
macro_rules! MetadataType {
    () => {
        # [doc = " LLVMMetadataType"] # [derive (Copy , Clone)] # [repr (C)] # [expect (dead_code , reason = "Some variants are unused, but are kept to match LLVM-C")] pub (crate) enum MetadataType { MD_dbg = 0 , MD_tbaa = 1 , MD_prof = 2 , MD_fpmath = 3 , MD_range = 4 , MD_tbaa_struct = 5 , MD_invariant_load = 6 , MD_alias_scope = 7 , MD_noalias = 8 , MD_nontemporal = 9 , MD_mem_parallel_loop_access = 10 , MD_nonnull = 11 , MD_unpredictable = 15 , MD_align = 17 , MD_type = 19 , MD_vcall_visibility = 28 , MD_noundef = 29 , MD_kcfi_type = 36 , }
    };
}

MetadataType!();
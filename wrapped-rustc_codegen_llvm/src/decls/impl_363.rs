macro_rules! deps {
    () => {
        DIB!();
        SmallVec!();
        Builder!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl < 'll > DebugInfoBuilderMethods for Builder < '_ , 'll , '_ > { fn dbg_var_addr (& mut self , dbg_var : & 'll DIVariable , dbg_loc : & 'll DILocation , variable_alloca : Self :: Value , direct_offset : Size , indirect_offsets : & [Size] , fragment : Option < Range < Size > > ,) { use dwarf_const :: { DW_OP_LLVM_fragment , DW_OP_deref , DW_OP_plus_uconst } ; let mut addr_ops = SmallVec :: < [u64 ; 8] > :: new () ; if direct_offset . bytes () > 0 { addr_ops . push (DW_OP_plus_uconst) ; addr_ops . push (direct_offset . bytes () as u64) ; } for & offset in indirect_offsets { addr_ops . push (DW_OP_deref) ; if offset . bytes () > 0 { addr_ops . push (DW_OP_plus_uconst) ; addr_ops . push (offset . bytes () as u64) ; } } if let Some (fragment) = fragment { addr_ops . push (DW_OP_LLVM_fragment) ; addr_ops . push (fragment . start . bits () as u64) ; addr_ops . push ((fragment . end - fragment . start) . bits () as u64) ; } unsafe { llvm :: LLVMRustDIBuilderInsertDeclareAtEnd (DIB (self . cx ()) , variable_alloca , dbg_var , addr_ops . as_ptr () , addr_ops . len () as c_uint , dbg_loc , self . llbb () ,) ; } } fn set_dbg_loc (& mut self , dbg_loc : & 'll DILocation) { unsafe { llvm :: LLVMSetCurrentDebugLocation2 (self . llbuilder , dbg_loc) ; } } fn clear_dbg_loc (& mut self) { unsafe { llvm :: LLVMSetCurrentDebugLocation2 (self . llbuilder , ptr :: null ()) ; } } fn insert_reference_to_gdb_debug_scripts_section_global (& mut self) { gdb :: insert_reference_to_gdb_debug_scripts_section_global (self) } fn set_var_name (& mut self , value : & 'll Value , name : & str) { if self . sess () . fewer_names () { return ; } let param_or_inst = unsafe { llvm :: LLVMIsAArgument (value) . is_some () || llvm :: LLVMIsAInstruction (value) . is_some () } ; if ! param_or_inst { return ; } if llvm :: get_value_name (value) . is_empty () { llvm :: set_value_name (value , name . as_bytes ()) ; } } }
    };
}

impl_363!()
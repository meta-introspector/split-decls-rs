macro_rules! deps {
    () => {
        GenericCx!();
        SCx!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { pub (crate) fn type_named_struct (& self , name : & str) -> & 'll Type { let name = SmallCStr :: new (name) ; unsafe { llvm :: LLVMStructCreateNamed (self . llcx () , name . as_ptr ()) } } pub (crate) fn set_struct_body (& self , ty : & 'll Type , els : & [& 'll Type] , packed : bool) { unsafe { llvm :: LLVMStructSetBody (ty , els . as_ptr () , els . len () as c_uint , packed . to_llvm_bool ()) } } pub (crate) fn type_void (& self) -> & 'll Type { unsafe { llvm :: LLVMVoidTypeInContext (self . llcx ()) } } # [doc = "x Creates an integer type with the given number of bits, e.g., i24"] pub (crate) fn type_ix (& self , num_bits : u64) -> & 'll Type { unsafe { llvm :: LLVMIntTypeInContext (self . llcx () , num_bits as c_uint) } } pub (crate) fn type_vector (& self , ty : & 'll Type , len : u64) -> & 'll Type { unsafe { llvm :: LLVMVectorType (ty , len as c_uint) } } pub (crate) fn func_params_types (& self , ty : & 'll Type) -> Vec < & 'll Type > { unsafe { let n_args = llvm :: LLVMCountParamTypes (ty) as usize ; let mut args = Vec :: with_capacity (n_args) ; llvm :: LLVMGetParamTypes (ty , args . as_mut_ptr ()) ; args . set_len (n_args) ; args } } }
    };
}

impl_581!()
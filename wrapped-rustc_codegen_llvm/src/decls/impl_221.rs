macro_rules! deps {
    () => {
        MetadataKindId!();
        SCx!();
        GenericCx!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { pub (crate) fn get_metadata_value (& self , metadata : & 'll Metadata) -> & 'll Value { llvm :: LLVMMetadataAsValue (self . llcx () , metadata) } pub (crate) fn get_const_int (& self , ty : & 'll Type , val : u64) -> & 'll Value { unsafe { llvm :: LLVMConstInt (ty , val , llvm :: FALSE) } } pub (crate) fn get_const_i64 (& self , n : u64) -> & 'll Value { self . get_const_int (self . type_i64 () , n) } pub (crate) fn get_const_i32 (& self , n : u64) -> & 'll Value { self . get_const_int (self . type_i32 () , n) } pub (crate) fn get_const_i16 (& self , n : u64) -> & 'll Value { self . get_const_int (self . type_i16 () , n) } pub (crate) fn get_const_i8 (& self , n : u64) -> & 'll Value { self . get_const_int (self . type_i8 () , n) } pub (crate) fn get_function (& self , name : & str) -> Option < & 'll Value > { let name = SmallCStr :: new (name) ; unsafe { llvm :: LLVMGetNamedFunction ((* * self) . borrow () . llmod , name . as_ptr ()) } } pub (crate) fn get_md_kind_id (& self , name : & str) -> llvm :: MetadataKindId { unsafe { llvm :: LLVMGetMDKindIDInContext (self . llcx () , name . as_ptr () as * const c_char , name . len () as c_uint ,) } } pub (crate) fn create_metadata (& self , name : & [u8]) -> & 'll Metadata { unsafe { llvm :: LLVMMDStringInContext2 (self . llcx () , name . as_ptr () as * const c_char , name . len ()) } } }
    };
}

impl_221!();
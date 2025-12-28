macro_rules! deps {
    () => {
        GenericCx!();
        SCx!();
        MetadataKindId!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { # [doc = " A wrapper for [`llvm::LLVMSetMetadata`], but it takes `Metadata` as a parameter instead of `Value`."] pub (crate) fn set_metadata < 'a > (& self , val : & 'a Value , kind_id : impl Into < llvm :: MetadataKindId > , md : & 'll Metadata ,) { let node = self . get_metadata_value (md) ; llvm :: LLVMSetMetadata (val , kind_id . into () , node) ; } }
    };
}

impl_225!();
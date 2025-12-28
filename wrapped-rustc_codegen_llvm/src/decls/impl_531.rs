macro_rules! deps {
    () => {
        Intrinsic!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl Intrinsic { pub (crate) fn lookup (name : & [u8]) -> Option < Self > { let id = unsafe { LLVMLookupIntrinsicID (name . as_c_char_ptr () , name . len ()) } ; NonZero :: new (id) . map (| id | Self { id }) } pub (crate) fn get_declaration < 'll > (self , llmod : & 'll Module , type_params : & [& 'll Type] ,) -> & 'll Value { unsafe { LLVMGetIntrinsicDeclaration (llmod , self . id , type_params . as_ptr () , type_params . len ()) } } }
    };
}

impl_531!()
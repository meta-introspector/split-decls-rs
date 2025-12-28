macro_rules! CreateAttrStringValue {
    () => {
        pub (crate) fn CreateAttrStringValue < 'll > (llcx : & 'll Context , attr : & str , value : & str ,) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , value . as_c_char_ptr () , value . len () . try_into () . unwrap () ,) } }
    };
}

CreateAttrStringValue!()
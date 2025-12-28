macro_rules! CreateAttrString {
    () => {
        pub (crate) fn CreateAttrString < 'll > (llcx : & 'll Context , attr : & str) -> & 'll Attribute { unsafe { LLVMCreateStringAttribute (llcx , attr . as_c_char_ptr () , attr . len () . try_into () . unwrap () , std :: ptr :: null () , 0 ,) } }
    };
}

CreateAttrString!();
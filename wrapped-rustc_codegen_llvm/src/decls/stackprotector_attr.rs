macro_rules! deps {
    () => {
        AttributeKind!();
        CodegenCx!();
    };
}

macro_rules! stackprotector_attr {
    () => {
        deps!();
        fn stackprotector_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { let sspattr = match cx . sess () . stack_protector () { StackProtector :: None => return None , StackProtector :: All => AttributeKind :: StackProtectReq , StackProtector :: Strong => AttributeKind :: StackProtectStrong , StackProtector :: Basic => AttributeKind :: StackProtect , } ; Some (sspattr . create_attr (cx . llcx)) }
    };
}

stackprotector_attr!();
macro_rules! deps {
    () => {
        CreateAttrString!();
        CodegenCx!();
    };
}

macro_rules! backchain_attr {
    () => {
        deps!();
        fn backchain_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { if cx . sess () . target . arch != "s390x" { return None ; } let requested_features = cx . sess () . opts . cg . target_feature . split (',') ; let found_positive = requested_features . clone () . any (| r | r == "+backchain") ; if found_positive { Some (llvm :: CreateAttrString (cx . llcx , "backchain")) } else { None } }
    };
}

backchain_attr!();
macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        CodegenCx!();
    };
}

macro_rules! nojumptables_attr {
    () => {
        deps!();
        fn nojumptables_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { if ! cx . sess () . opts . unstable_opts . no_jump_tables { return None ; } Some (llvm :: CreateAttrStringValue (cx . llcx , "no-jump-tables" , "true")) }
    };
}

nojumptables_attr!()
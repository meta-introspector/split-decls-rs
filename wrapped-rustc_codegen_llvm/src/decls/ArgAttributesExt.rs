macro_rules! deps {
    () => {
        CodegenCx!();
        AttributePlace!();
    };
}

macro_rules! ArgAttributesExt {
    () => {
        deps!();
        trait ArgAttributesExt { fn apply_attrs_to_llfn (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , llfn : & Value) ; fn apply_attrs_to_callsite (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , callsite : & Value ,) ; }
    };
}

ArgAttributesExt!()
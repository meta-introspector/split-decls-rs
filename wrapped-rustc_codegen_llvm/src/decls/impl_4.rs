macro_rules! deps {
    () => {
        CodegenCx!();
        AttributePlace!();
        ArgAttributesExt!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl ArgAttributesExt for ArgAttributes { fn apply_attrs_to_llfn (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , llfn : & Value) { let attrs = get_attrs (self , cx) ; attributes :: apply_to_llfn (llfn , idx , & attrs) ; } fn apply_attrs_to_callsite (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , callsite : & Value ,) { let attrs = get_attrs (self , cx) ; attributes :: apply_to_callsite (callsite , idx , & attrs) ; } }
    };
}

impl_4!()
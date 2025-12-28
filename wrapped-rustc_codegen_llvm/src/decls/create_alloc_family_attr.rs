macro_rules! deps {
    () => {
        CreateAttrStringValue!();
    };
}

macro_rules! create_alloc_family_attr {
    () => {
        deps!();
        fn create_alloc_family_attr (llcx : & llvm :: Context) -> & llvm :: Attribute { llvm :: CreateAttrStringValue (llcx , "alloc-family" , "__rust_alloc") }
    };
}

create_alloc_family_attr!();
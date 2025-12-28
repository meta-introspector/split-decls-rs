macro_rules! deps {
    () => {
        CodegenCx!();
        AttributeKind!();
    };
}

macro_rules! non_lazy_bind_attr {
    () => {
        deps!();
        # [doc = " Get the `NonLazyBind` LLVM attribute,"] # [doc = " if the codegen options allow skipping the PLT."] pub (crate) fn non_lazy_bind_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { if ! cx . sess () . needs_plt () { Some (AttributeKind :: NonLazyBind . create_attr (cx . llcx)) } else { None } }
    };
}

non_lazy_bind_attr!()
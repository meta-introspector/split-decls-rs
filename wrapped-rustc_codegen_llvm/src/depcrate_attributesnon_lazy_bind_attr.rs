// Generated macro for non_lazy_bind_attr (function)
macro_rules! Depcrate_attributesnon_lazy_bind_attr {
() => {
// Module: crate::attributes
// Provides: {"non_lazy_bind_attr"}
// Dependencies: {}
# [doc = " Get the `NonLazyBind` LLVM attribute,"] # [doc = " if the codegen options allow skipping the PLT."] pub (crate) fn non_lazy_bind_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { if ! cx . sess () . needs_plt () { Some (AttributeKind :: NonLazyBind . create_attr (cx . llcx)) } else { None } }
};
}

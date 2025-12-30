// Generated macro for PathList (struct)
macro_rules! Depcrate_util_path_listPathList {
() => {
// Module: crate::util::path_list
// Provides: {"PathList"}
// Dependencies: {}
# [doc = " A list of `syn::Path` instances. This type is used to extract a list of paths from an"] # [doc = " attribute."] # [doc = ""] # [doc = " # Usage"] # [doc = " An `PathList` field on a struct implementing `FromMeta` will turn `#[builder(derive(serde::Debug, Clone))]` into:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " StructOptions {"] # [doc = "     derive: PathList(vec![syn::Path::new(\"serde::Debug\"), syn::Path::new(\"Clone\")])"] # [doc = " }"] # [doc = " ```"] # [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct PathList (Vec < Path >) ;
};
}

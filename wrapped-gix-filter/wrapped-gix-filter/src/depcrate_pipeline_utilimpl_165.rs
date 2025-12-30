// Generated macro for impl_165 (impl)
macro_rules! Depcrate_pipeline_utilimpl_165 {
() => {
// Module: crate::pipeline::util
// Provides: {"impl_165"}
// Dependencies: {}
impl Context { pub (crate) fn with_path < 'a > (& self , rela_path : & 'a BStr) -> driver :: apply :: Context < 'a , '_ > { driver :: apply :: Context { rela_path , ref_name : self . ref_name . as_ref () . map (AsRef :: as_ref) , treeish : self . treeish , blob : self . blob , } } }
};
}

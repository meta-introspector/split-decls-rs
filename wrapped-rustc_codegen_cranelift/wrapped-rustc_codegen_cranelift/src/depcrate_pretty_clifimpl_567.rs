// Generated macro for impl_567 (impl)
macro_rules! Depcrate_pretty_clifimpl_567 {
() => {
// Module: crate::pretty_clif
// Provides: {"impl_567"}
// Dependencies: {}
impl FunctionCx < '_ , '_ , '_ > { pub (crate) fn add_global_comment < S : Into < String > > (& mut self , comment : S) { self . clif_comments . add_global_comment (comment) ; } pub (crate) fn add_comment < S : Into < String > + AsRef < str > , E : Into < AnyEntity > > (& mut self , entity : E , comment : S ,) { self . clif_comments . add_comment (entity , comment) ; } pub (crate) fn add_post_comment < S : Into < String > + AsRef < str > > (& mut self , entity : Inst , comment : S ,) { self . clif_comments . add_post_comment (entity , comment) ; } }
};
}

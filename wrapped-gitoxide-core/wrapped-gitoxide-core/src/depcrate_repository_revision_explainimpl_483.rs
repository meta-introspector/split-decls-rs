// Generated macro for impl_483 (impl)
macro_rules! Depcrate_repository_revision_explainimpl_483 {
() => {
// Module: crate::repository::revision::explain
// Provides: {"impl_483"}
// Dependencies: {}
impl < 'a > Explain < 'a > { fn new (out : & 'a mut impl std :: io :: Write) -> Self { Explain { out , call : 0 , ref_name : None , oid_prefix : None , has_implicit_anchor : false , err : None , } } fn prefix (& mut self) -> Option < () > { self . call += 1 ; write ! (self . out , "{:02}. " , self . call) . ok () } fn revision_name (& self) -> BString { self . ref_name . clone () . unwrap_or_else (| | { self . oid_prefix . expect ("parser must have set some object value") . to_string () . into () }) } }
};
}

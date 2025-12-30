// Generated macro for impl_118 (impl)
macro_rules! Depcrate_canonicalimpl_118 {
() => {
// Module: crate::canonical
// Provides: {"impl_118"}
// Dependencies: {}
impl < A > NodeSet < A > { fn insert_if_absent (& mut self , id : String , or : Node < A >) { if self . set . get (& id) . is_none () { self . set . insert (id , or) ; } } fn map < F , B > (self , f : F) -> NodeSet < B > where F : Fn (A) -> Option < B > , { let new_set = self . set . into_iter () . map (| (name , node) | (name , node . map (& f))) . collect () ; NodeSet { set : new_set } } }
};
}

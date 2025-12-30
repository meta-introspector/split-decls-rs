// Generated macro for objects (macro)
macro_rules! Depcrate_rt_objectobjects {
() => {
// Module: crate::rt::object
// Provides: {"objects"}
// Dependencies: {}
macro_rules ! objects { ($ (# [$ attrs : meta]) * $ e : ident , $ ($ name : ident ($ ty : path) ,) *) => { $ (# [$ attrs]) * pub (super) enum $ e { $ ($ name ($ ty) ,) * } $ (impl crate :: rt :: object :: Object for $ ty { type Entry = $ e ; fn into_entry (self) -> Entry { $ e ::$ name (self) } fn get_ref (entry : & Entry) -> Option <&$ ty > { match entry { $ e ::$ name (obj) => Some (obj) , _ => None , } } fn get_mut (entry : & mut Entry) -> Option <& mut $ ty > { match entry { $ e ::$ name (obj) => Some (obj) , _ => None , } } }) * } ; }
};
}

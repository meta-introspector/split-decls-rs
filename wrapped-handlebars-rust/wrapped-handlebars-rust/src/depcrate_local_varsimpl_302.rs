// Generated macro for impl_302 (impl)
macro_rules! Depcrate_local_varsimpl_302 {
() => {
// Module: crate::local_vars
// Provides: {"impl_302"}
// Dependencies: {}
impl LocalVars { pub fn put (& mut self , key : & str , value : Json) { match key { "first" => self . first = Some (value) , "last" => self . last = Some (value) , "index" => self . index = Some (value) , "key" => self . key = Some (value) , _ => { self . extra . insert (key . to_owned () , value) ; } } } pub fn get (& self , key : & str) -> Option < & Json > { match key { "first" => self . first . as_ref () , "last" => self . last . as_ref () , "index" => self . index . as_ref () , "key" => self . key . as_ref () , _ => self . extra . get (key) , } } }
};
}

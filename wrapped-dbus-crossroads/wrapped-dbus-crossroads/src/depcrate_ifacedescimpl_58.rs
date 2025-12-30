// Generated macro for impl_58 (impl)
macro_rules! Depcrate_ifacedescimpl_58 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_58"}
// Dependencies: {}
impl Arguments { fn introspect (& self , dir : Option < & str > , prefix : & str) -> String { let mut r = String :: new () ; for a in & self . 0 { r += & format ! ("{}<arg name=\"{}\" type=\"{}\"" , prefix , a . name , a . sig) ; if let Some (dir) = dir { r += & format ! (" direction=\"{}\"" , dir) ; } if a . annotations . is_empty () { r += "/>\n" ; } else { let inner_prefix = format ! ("{}  " , prefix) ; r += & format ! (">\n{}{}</arg>\n" , a . annotations . introspect (& inner_prefix) , prefix) ; } } r } }
};
}

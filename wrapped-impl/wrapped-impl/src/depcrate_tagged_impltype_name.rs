// Generated macro for type_name (function)
macro_rules! Depcrate_tagged_impltype_name {
() => {
// Module: crate::tagged_impl
// Provides: {"type_name"}
// Dependencies: {}
fn type_name (mut ty : & Type) -> Option < String > { loop { match ty { Type :: Path (TypePath { qself : None , path }) => { return Some (path . segments . last () . unwrap () . ident . to_string ()) ; } Type :: Group (group) => { ty = & group . elem ; } _ => return None , } } }
};
}

// Generated macro for impl_148 (impl)
macro_rules! Depcrate_tag_ref_iterimpl_148 {
() => {
// Module: crate::tag::ref_iter
// Provides: {"impl_148"}
// Dependencies: {}
impl Token < '_ > { # [doc = " Return the object id of this token if its a [Target][Token::Target]."] pub fn id (& self) -> Option < & oid > { match self { Token :: Target { id } => Some (id . as_ref ()) , _ => None , } } # [doc = " Return the owned object id of this token if its a [Target][Token::Target]."] pub fn into_id (self) -> Option < ObjectId > { match self { Token :: Target { id } => Some (id) , _ => None , } } }
};
}

// Generated macro for impl_69 (impl)
macro_rules! Depcrate_commit_ref_iterimpl_69 {
() => {
// Module: crate::commit::ref_iter
// Provides: {"impl_69"}
// Dependencies: {}
impl Token < '_ > { # [doc = " Return the object id of this token if it's a [tree][Token::Tree] or a [parent commit][Token::Parent]."] pub fn id (& self) -> Option < & oid > { match self { Token :: Tree { id } | Token :: Parent { id } => Some (id . as_ref ()) , _ => None , } } # [doc = " Return the owned object id of this token if it's a [tree][Token::Tree] or a [parent commit][Token::Parent]."] pub fn try_into_id (self) -> Option < ObjectId > { match self { Token :: Tree { id } | Token :: Parent { id } => Some (id) , _ => None , } } }
};
}

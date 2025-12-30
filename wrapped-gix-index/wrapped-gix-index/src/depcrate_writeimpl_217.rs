// Generated macro for impl_217 (impl)
macro_rules! Depcrate_writeimpl_217 {
() => {
// Module: crate::write
// Provides: {"impl_217"}
// Dependencies: {}
impl Extensions { # [doc = " Returns `Some(signature)` if it should be written out."] pub fn should_write (& self , signature : extension :: Signature) -> Option < extension :: Signature > { match self { Extensions :: None => None , Extensions :: All => Some (signature) , Extensions :: Given { tree_cache , end_of_index_entry , } => match signature { extension :: tree :: SIGNATURE => tree_cache , extension :: end_of_index_entry :: SIGNATURE => end_of_index_entry , _ => & false , } . then (| | signature) , } } }
};
}

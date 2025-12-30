// Generated macro for impl_152 (impl)
macro_rules! Depcrate_handshake_refs_sharedimpl_152 {
() => {
// Module: crate::handshake::refs::shared
// Provides: {"impl_152"}
// Dependencies: {}
impl InternalRef { fn unpack_direct (self) -> Option < (BString , gix_hash :: ObjectId) > { match self { InternalRef :: Direct { path , object } => Some ((path , object)) , _ => None , } } fn lookup_symbol_has_path (& self , predicate_path : & BStr) -> bool { matches ! (self , InternalRef :: SymbolicForLookup { path , .. } if path == predicate_path) } }
};
}

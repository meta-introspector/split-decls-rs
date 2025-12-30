// Generated macro for change_impls (module)
macro_rules! Depcrate_tree_visitchange_impls {
() => {
// Module: crate::tree::visit
// Provides: {"change_impls"}
// Dependencies: {}
# [cfg (feature = "blob")] mod change_impls { use gix_hash :: oid ; use gix_object :: tree :: EntryMode ; use crate :: { rewrites :: tracker :: ChangeKind , tree :: visit :: { Change , Relation } , } ; impl crate :: rewrites :: tracker :: Change for crate :: tree :: visit :: Change { fn id (& self) -> & oid { match self { Change :: Addition { oid , .. } | Change :: Deletion { oid , .. } | Change :: Modification { oid , .. } => oid , } } fn relation (& self) -> Option < Relation > { match self { Change :: Addition { relation , .. } | Change :: Deletion { relation , .. } => * relation , Change :: Modification { .. } => None , } } fn kind (& self) -> ChangeKind { match self { Change :: Addition { .. } => ChangeKind :: Addition , Change :: Deletion { .. } => ChangeKind :: Deletion , Change :: Modification { .. } => ChangeKind :: Modification , } } fn entry_mode (& self) -> EntryMode { match self { Change :: Addition { entry_mode , .. } | Change :: Deletion { entry_mode , .. } | Change :: Modification { entry_mode , .. } => * entry_mode , } } fn id_and_entry_mode (& self) -> (& oid , EntryMode) { match self { Change :: Addition { entry_mode , oid , .. } | Change :: Deletion { entry_mode , oid , .. } | Change :: Modification { entry_mode , oid , .. } => (oid , * entry_mode) , } } } }
};
}

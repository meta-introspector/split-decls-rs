// Generated macro for ParentedNode (struct)
macro_rules! Depcrate_hirParentedNode {
() => {
// Module: crate::hir
// Provides: {"ParentedNode"}
// Dependencies: {}
# [doc = " HIR node coupled with its parent's id in the same HIR owner."] # [doc = ""] # [doc = " The parent is trash when the node is a HIR owner."] # [derive (Clone , Copy , Debug)] pub struct ParentedNode < 'tcx > { pub parent : ItemLocalId , pub node : Node < 'tcx > , }
};
}

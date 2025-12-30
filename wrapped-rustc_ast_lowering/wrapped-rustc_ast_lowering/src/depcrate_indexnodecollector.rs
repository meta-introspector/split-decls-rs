// Generated macro for NodeCollector (struct)
macro_rules! Depcrate_indexNodeCollector {
() => {
// Module: crate::index
// Provides: {"NodeCollector"}
// Dependencies: {}
# [doc = " A visitor that walks over the HIR and collects `Node`s into a HIR map."] struct NodeCollector < 'a , 'hir > { tcx : TyCtxt < 'hir > , bodies : & 'a SortedMap < ItemLocalId , & 'hir Body < 'hir > > , # [doc = " Outputs"] nodes : IndexVec < ItemLocalId , ParentedNode < 'hir > > , parenting : LocalDefIdMap < ItemLocalId > , # [doc = " The parent of this node"] parent_node : ItemLocalId , owner : OwnerId , }
};
}

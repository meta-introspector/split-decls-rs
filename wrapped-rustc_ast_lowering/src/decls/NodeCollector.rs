macro_rules! NodeCollector {
    () => {
        # [doc = " A visitor that walks over the HIR and collects `Node`s into a HIR map."] struct NodeCollector < 'a , 'hir > { tcx : TyCtxt < 'hir > , bodies : & 'a SortedMap < ItemLocalId , & 'hir Body < 'hir > > , # [doc = " Outputs"] nodes : IndexVec < ItemLocalId , ParentedNode < 'hir > > , parenting : LocalDefIdMap < ItemLocalId > , # [doc = " The parent of this node"] parent_node : ItemLocalId , owner : OwnerId , }
    };
}

NodeCollector!()
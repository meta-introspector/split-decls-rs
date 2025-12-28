macro_rules! deps {
    () => {
        NodeCollector!();
    };
}

macro_rules! index_hir {
    () => {
        deps!();
        # [instrument (level = "debug" , skip (tcx , bodies))] pub (super) fn index_hir < 'hir > (tcx : TyCtxt < 'hir > , item : hir :: OwnerNode < 'hir > , bodies : & SortedMap < ItemLocalId , & 'hir Body < 'hir > > , num_nodes : usize ,) -> (IndexVec < ItemLocalId , ParentedNode < 'hir > > , LocalDefIdMap < ItemLocalId >) { let err_node = ParentedNode { parent : ItemLocalId :: ZERO , node : Node :: Err (item . span ()) } ; let mut nodes = IndexVec :: from_elem_n (err_node , num_nodes) ; nodes [ItemLocalId :: ZERO] = ParentedNode { parent : ItemLocalId :: INVALID , node : item . into () } ; let mut collector = NodeCollector { tcx , owner : item . def_id () , parent_node : ItemLocalId :: ZERO , nodes , bodies , parenting : Default :: default () , } ; match item { OwnerNode :: Crate (citem) => { collector . visit_mod (citem , citem . spans . inner_span , hir :: CRATE_HIR_ID) } OwnerNode :: Item (item) => collector . visit_item (item) , OwnerNode :: TraitItem (item) => collector . visit_trait_item (item) , OwnerNode :: ImplItem (item) => collector . visit_impl_item (item) , OwnerNode :: ForeignItem (item) => collector . visit_foreign_item (item) , OwnerNode :: Synthetic => unreachable ! () , } ; for (local_id , node) in collector . nodes . iter_enumerated () { if let Node :: Err (span) = node . node { let hir_id = HirId { owner : item . def_id () , local_id } ; let msg = format ! ("ID {hir_id} not encountered when visiting item HIR") ; tcx . dcx () . span_delayed_bug (span , msg) ; } } (collector . nodes , collector . parenting) }
    };
}

index_hir!();
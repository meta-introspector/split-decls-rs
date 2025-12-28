macro_rules! deps {
    () => {
        LoweringContext!();
        ItemLowerer!();
        AstOwner!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'a , 'hir > ItemLowerer < 'a , 'hir > { fn with_lctx (& mut self , owner : NodeId , f : impl FnOnce (& mut LoweringContext < '_ , 'hir >) -> hir :: OwnerNode < 'hir > ,) { let mut lctx = LoweringContext :: new (self . tcx , self . resolver) ; lctx . with_hir_id_owner (owner , | lctx | f (lctx)) ; for (def_id , info) in lctx . children { let owner = self . owners . ensure_contains_elem (def_id , | | hir :: MaybeOwner :: Phantom) ; assert ! (matches ! (owner , hir :: MaybeOwner :: Phantom) , "duplicate copy of {def_id:?} in lctx.children") ; * owner = info ; } } pub (super) fn lower_node (& mut self , def_id : LocalDefId) { let owner = self . owners . ensure_contains_elem (def_id , | | hir :: MaybeOwner :: Phantom) ; if let hir :: MaybeOwner :: Phantom = owner { let node = self . ast_index [def_id] ; match node { AstOwner :: NonOwner => { } AstOwner :: Crate (c) => { assert_eq ! (self . resolver . node_id_to_def_id [& CRATE_NODE_ID] , CRATE_DEF_ID) ; self . with_lctx (CRATE_NODE_ID , | lctx | { let module = lctx . lower_mod (& c . items , & c . spans) ; lctx . lower_attrs (hir :: CRATE_HIR_ID , & c . attrs , DUMMY_SP , Target :: Crate) ; hir :: OwnerNode :: Crate (module) }) } AstOwner :: Item (item) => { self . with_lctx (item . id , | lctx | hir :: OwnerNode :: Item (lctx . lower_item (item))) } AstOwner :: AssocItem (item , ctxt) => { self . with_lctx (item . id , | lctx | lctx . lower_assoc_item (item , ctxt)) } AstOwner :: ForeignItem (item) => self . with_lctx (item . id , | lctx | { hir :: OwnerNode :: ForeignItem (lctx . lower_foreign_item (item)) }) , } } } }
    };
}

impl_81!()
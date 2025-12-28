macro_rules! deps {
    () => {
        AstOwner!();
    };
}

macro_rules! index_crate {
    () => {
        deps!();
        fn index_crate < 'a > (node_id_to_def_id : & NodeMap < LocalDefId > , krate : & 'a Crate ,) -> IndexVec < LocalDefId , AstOwner < 'a > > { let mut indexer = Indexer { node_id_to_def_id , index : IndexVec :: new () } ; * indexer . index . ensure_contains_elem (CRATE_DEF_ID , | | AstOwner :: NonOwner) = AstOwner :: Crate (krate) ; visit :: walk_crate (& mut indexer , krate) ; return indexer . index ; struct Indexer < 's , 'a > { node_id_to_def_id : & 's NodeMap < LocalDefId > , index : IndexVec < LocalDefId , AstOwner < 'a > > , } impl < 'a > visit :: Visitor < 'a > for Indexer < '_ , 'a > { fn visit_attribute (& mut self , _ : & 'a Attribute) { } fn visit_item (& mut self , item : & 'a ast :: Item) { let def_id = self . node_id_to_def_id [& item . id] ; * self . index . ensure_contains_elem (def_id , | | AstOwner :: NonOwner) = AstOwner :: Item (item) ; visit :: walk_item (self , item) } fn visit_assoc_item (& mut self , item : & 'a ast :: AssocItem , ctxt : visit :: AssocCtxt) { let def_id = self . node_id_to_def_id [& item . id] ; * self . index . ensure_contains_elem (def_id , | | AstOwner :: NonOwner) = AstOwner :: AssocItem (item , ctxt) ; visit :: walk_assoc_item (self , item , ctxt) ; } fn visit_foreign_item (& mut self , item : & 'a ast :: ForeignItem) { let def_id = self . node_id_to_def_id [& item . id] ; * self . index . ensure_contains_elem (def_id , | | AstOwner :: NonOwner) = AstOwner :: ForeignItem (item) ; visit :: walk_item (self , item) ; } } }
    };
}

index_crate!()
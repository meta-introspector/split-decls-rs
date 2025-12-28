macro_rules! deps {
    () => {
        NodeCollector!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a , 'hir > NodeCollector < 'a , 'hir > { # [instrument (level = "debug" , skip (self))] fn insert (& mut self , span : Span , hir_id : HirId , node : Node < 'hir >) { debug_assert_eq ! (self . owner , hir_id . owner) ; debug_assert_ne ! (hir_id . local_id . as_u32 () , 0) ; debug_assert_ne ! (hir_id . local_id , self . parent_node) ; if cfg ! (debug_assertions) { if hir_id . owner != self . owner { span_bug ! (span , "inconsistent HirId at `{:?}` for `{node:?}`: \
                     current_dep_node_owner={} ({:?}), hir_id.owner={} ({:?})" , self . tcx . sess . source_map () . span_to_diagnostic_string (span) , self . tcx . definitions_untracked () . def_path (self . owner . def_id) . to_string_no_crate_verbose () , self . owner , self . tcx . definitions_untracked () . def_path (hir_id . owner . def_id) . to_string_no_crate_verbose () , hir_id . owner ,) } if self . tcx . sess . opts . incremental . is_some () && span . parent () . is_none () && ! span . is_dummy () { span_bug ! (span , "span without a parent: {:#?}, {node:?}" , span . data ()) } } self . nodes [hir_id . local_id] = ParentedNode { parent : self . parent_node , node } ; } fn with_parent < F : FnOnce (& mut Self) > (& mut self , parent_node_id : HirId , f : F) { debug_assert_eq ! (parent_node_id . owner , self . owner) ; let parent_node = self . parent_node ; self . parent_node = parent_node_id . local_id ; f (self) ; self . parent_node = parent_node ; } fn insert_nested (& mut self , item : LocalDefId) { if self . parent_node != ItemLocalId :: ZERO { self . parenting . insert (item , self . parent_node) ; } } }
    };
}

impl_76!()
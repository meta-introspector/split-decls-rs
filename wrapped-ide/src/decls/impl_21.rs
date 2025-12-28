macro_rules! deps {
    () => {
        NavigationTarget!();
        TryToNav!();
        UpmappingResult!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl TryToNav for FileSymbol { fn try_to_nav (& self , sema : & Semantics < '_ , RootDatabase > ,) -> Option < UpmappingResult < NavigationTarget > > { let db = sema . db ; let display_target = self . def . krate (db) . to_display_target (db) ; Some (orig_range_with_focus_r (db , self . loc . hir_file_id , self . loc . ptr . text_range () , Some (self . loc . name_ptr . text_range ()) ,) . map (| (FileRange { file_id , range : full_range } , focus_range) | { NavigationTarget { file_id , name : self . is_alias . then (| | self . def . name (db)) . flatten () . map_or_else (| | self . name . clone () , | it | it . symbol () . clone ()) , alias : self . is_alias . then (| | self . name . clone ()) , kind : Some (self . def . into ()) , full_range , focus_range , container_name : self . container_name . clone () , description : match self . def { hir :: ModuleDef :: Module (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: Function (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: Adt (it) => Some (it . display (db , display_target) . to_string ()) , hir :: ModuleDef :: Variant (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: Const (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: Static (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: Trait (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: TypeAlias (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: Macro (it) => { Some (it . display (db , display_target) . to_string ()) } hir :: ModuleDef :: BuiltinType (_) => None , } , docs : None , } }) ,) } }
    };
}

impl_21!()
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_middle :: query :: LocalCrate ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: cstore :: ForeignModule ;}

macro_rules! collect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect in module {}", module_path!());
    };
}

mkfn!{
    collect_introspect!();
    pub (crate) fn collect (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> FxIndexMap < DefId , ForeignModule > { let mut modules = FxIndexMap :: default () ; for id in tcx . hir_free_items () { if ! matches ! (tcx . def_kind (id . owner_id) , DefKind :: ForeignMod) { continue ; } let def_id = id . owner_id . to_def_id () ; let item = tcx . hir_item (id) ; if let hir :: ItemKind :: ForeignMod { abi , items } = item . kind { let foreign_items = items . iter () . map (| it | it . owner_id . to_def_id ()) . collect () ; modules . insert (def_id , ForeignModule { def_id , abi , foreign_items }) ; } } modules }
}
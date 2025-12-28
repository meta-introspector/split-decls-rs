macro_rules! deps {
    () => {
        HoverAction!();
        HoverGotoTypeData!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl HoverAction { fn goto_type_from_targets (sema : & Semantics < '_ , RootDatabase > , targets : Vec < hir :: ModuleDef > , edition : Edition ,) -> Option < Self > { let db = sema . db ; let targets = targets . into_iter () . filter_map (| it | { Some (HoverGotoTypeData { mod_path : render :: path (db , it . module (db) ? , it . name (db) . map (| name | name . display (db , edition) . to_string ()) , edition ,) , nav : it . try_to_nav (sema) ? . call_site () , }) }) . collect :: < Vec < _ > > () ; targets . is_empty () . not () . then_some (HoverAction :: GoToType (targets)) } }
    };
}

impl_213!();
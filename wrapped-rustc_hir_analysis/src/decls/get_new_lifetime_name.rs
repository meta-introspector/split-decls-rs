macro_rules! get_new_lifetime_name {
    () => {
        # [doc = " Synthesize a new lifetime name that doesn't clash with any of the lifetimes already present."] fn get_new_lifetime_name < 'tcx > (tcx : TyCtxt < 'tcx > , poly_trait_ref : ty :: PolyTraitRef < 'tcx > , generics : & hir :: Generics < 'tcx > ,) -> String { let existing_lifetimes = tcx . collect_referenced_late_bound_regions (poly_trait_ref) . into_iter () . filter_map (| lt | lt . get_name (tcx) . map (| name | name . as_str () . to_string ())) . chain (generics . params . iter () . filter_map (| param | { if let hir :: GenericParamKind :: Lifetime { .. } = & param . kind { Some (param . name . ident () . as_str () . to_string ()) } else { None } })) . collect :: < FxHashSet < String > > () ; let a_to_z_repeat_n = | n | { (b'a' ..= b'z') . map (move | c | { let mut s = '\'' . to_string () ; s . extend (std :: iter :: repeat (char :: from (c)) . take (n)) ; s }) } ; (1 ..) . flat_map (a_to_z_repeat_n) . find (| lt | ! existing_lifetimes . contains (lt . as_str ())) . unwrap () }
    };
}

get_new_lifetime_name!()
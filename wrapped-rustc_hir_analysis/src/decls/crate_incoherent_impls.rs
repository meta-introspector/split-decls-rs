macro_rules! crate_incoherent_impls {
    () => {
        pub (crate) fn crate_incoherent_impls (tcx : TyCtxt < '_ > , simp : SimplifiedType) -> & [DefId] { let (crate_map , _) = tcx . crate_inherent_impls (()) ; tcx . arena . alloc_from_iter (crate_map . incoherent_impls . get (& simp) . unwrap_or (& Vec :: new ()) . iter () . map (| d | d . to_def_id ()) ,) }
    };
}

crate_incoherent_impls!()
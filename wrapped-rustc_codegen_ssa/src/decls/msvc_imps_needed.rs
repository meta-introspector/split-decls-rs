macro_rules! msvc_imps_needed {
    () => {
        fn msvc_imps_needed (tcx : TyCtxt < '_ >) -> bool { assert ! (! (tcx . sess . opts . cg . linker_plugin_lto . enabled () && tcx . sess . target . is_like_windows && tcx . sess . opts . cg . prefer_dynamic)) ; let can_have_static_objects = tcx . sess . lto () == Lto :: Thin || tcx . crate_types () . contains (& CrateType :: Rlib) ; tcx . sess . target . is_like_windows && can_have_static_objects && ! tcx . sess . opts . cg . linker_plugin_lto . enabled () }
    };
}

msvc_imps_needed!();
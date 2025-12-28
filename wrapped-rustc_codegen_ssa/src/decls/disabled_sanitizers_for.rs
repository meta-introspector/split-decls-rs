macro_rules! disabled_sanitizers_for {
    () => {
        fn disabled_sanitizers_for (tcx : TyCtxt < '_ > , did : LocalDefId) -> SanitizerSet { let mut disabled = match tcx . opt_local_parent (did) { Some (parent) => tcx . disabled_sanitizers_for (parent) , None => SanitizerSet :: empty () , } ; if let Some ((on_set , off_set)) = find_attr ! (tcx . get_all_attrs (did) , AttributeKind :: Sanitize { on_set , off_set , .. } => (on_set , off_set)) { disabled &= ! * on_set ; disabled |= * off_set ; } disabled }
    };
}

disabled_sanitizers_for!()
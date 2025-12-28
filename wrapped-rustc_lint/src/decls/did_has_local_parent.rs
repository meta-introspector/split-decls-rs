macro_rules! did_has_local_parent {
    () => {
        # [doc = " Given a def id this checks if the parent def id (modulo modules) correspond to"] # [doc = " the def id of the parent impl definition (the direct one and the outermost one)."] # [inline] fn did_has_local_parent (did : DefId , tcx : TyCtxt < '_ > , impl_parent : DefId , outermost_impl_parent : Option < DefId > ,) -> bool { if ! did . is_local () { return false ; } let Some (parent_did) = tcx . opt_parent (did) else { return false ; } ; peel_parent_while (tcx , parent_did , | tcx , did | { tcx . def_kind (did) == DefKind :: Mod || (tcx . def_kind (did) == DefKind :: Const && tcx . opt_item_name (did) == Some (kw :: Underscore)) }) . map (| parent_did | parent_did == impl_parent || Some (parent_did) == outermost_impl_parent) . unwrap_or (false) }
    };
}

did_has_local_parent!()
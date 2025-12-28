macro_rules! check_packed_inner {
    () => {
        pub (super) fn check_packed_inner (tcx : TyCtxt < '_ > , def_id : DefId , stack : & mut Vec < DefId > ,) -> Option < Vec < (DefId , Span) > > { if let ty :: Adt (def , args) = tcx . type_of (def_id) . instantiate_identity () . kind () { if def . is_struct () || def . is_union () { if def . repr () . align . is_some () { return Some (vec ! [(def . did () , DUMMY_SP)]) ; } stack . push (def_id) ; for field in & def . non_enum_variant () . fields { if let ty :: Adt (def , _) = field . ty (tcx , args) . kind () && ! stack . contains (& def . did ()) && let Some (mut defs) = check_packed_inner (tcx , def . did () , stack) { defs . push ((def . did () , field . ident (tcx) . span)) ; return Some (defs) ; } } stack . pop () ; } } None }
    };
}

check_packed_inner!()
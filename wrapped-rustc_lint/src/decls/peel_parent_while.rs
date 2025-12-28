macro_rules! peel_parent_while {
    () => {
        # [doc = " Given a `DefId` checks if it satisfies `f` if it does check with it's parent and continue"] # [doc = " until it doesn't satisfies `f` and return the last `DefId` checked."] # [doc = ""] # [doc = " In other word this method return the first `DefId` that doesn't satisfies `f`."] # [inline] fn peel_parent_while (tcx : TyCtxt < '_ > , mut did : DefId , mut f : impl FnMut (TyCtxt < '_ > , DefId) -> bool ,) -> Option < DefId > { while ! did . is_crate_root () && f (tcx , did) { did = tcx . opt_parent (did) . filter (| parent_did | parent_did . is_local ()) ? ; } Some (did) }
    };
}

peel_parent_while!();
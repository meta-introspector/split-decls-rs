macro_rules! deps {
    () => {
        FnCtxt!();
        Locatable!();
    };
}

macro_rules! Resolver {
    () => {
        deps!();
        struct Resolver < 'cx , 'tcx > { fcx : & 'cx FnCtxt < 'cx , 'tcx > , span : & 'cx dyn Locatable , body : & 'tcx hir :: Body < 'tcx > , # [doc = " Whether we should normalize using the new solver, disabled"] # [doc = " both when using the old solver and when resolving predicates."] should_normalize : bool , nested_goals : & 'cx mut Vec < Goal < 'tcx , ty :: Predicate < 'tcx > > > , }
    };
}

Resolver!();
macro_rules! DropTraitConstraintsDiag {
    () => {
        pub (crate) struct DropTraitConstraintsDiag < 'a > { pub predicate : Clause < 'a > , pub tcx : TyCtxt < 'a > , pub def_id : DefId , }
    };
}

DropTraitConstraintsDiag!()
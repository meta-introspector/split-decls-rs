macro_rules! deps {
    () => {
        HirDatabase!();
        AllowSelfProjection!();
    };
}

macro_rules! predicate_references_self {
    () => {
        deps!();
        fn predicate_references_self < 'db > (db : & 'db dyn HirDatabase , trait_ : TraitId , predicate : Clause < 'db > , allow_self_projection : AllowSelfProjection ,) -> bool { match predicate . kind () . skip_binder () { ClauseKind :: Trait (trait_pred) => trait_pred . trait_ref . args . iter () . skip (1) . any (| arg | { contains_illegal_self_type_reference (db , trait_ , & arg , allow_self_projection) }) , ClauseKind :: Projection (proj_pred) => { proj_pred . projection_term . args . iter () . skip (1) . any (| arg | { contains_illegal_self_type_reference (db , trait_ , & arg , allow_self_projection) }) } _ => false , } }
    };
}

predicate_references_self!()
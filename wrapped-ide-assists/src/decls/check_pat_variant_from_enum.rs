macro_rules! deps {
    () => {
        AssistContext!();
    };
}

macro_rules! check_pat_variant_from_enum {
    () => {
        deps!();
        fn check_pat_variant_from_enum (ctx : & AssistContext < '_ > , pat : & ast :: Pat) -> bool { ctx . sema . type_of_pat (pat) . is_none_or (| ty : hir :: TypeInfo < '_ > | { ty . adjusted () . as_adt () . is_some_and (| adt | matches ! (adt , hir :: Adt :: Enum (_))) }) }
    };
}

check_pat_variant_from_enum!()
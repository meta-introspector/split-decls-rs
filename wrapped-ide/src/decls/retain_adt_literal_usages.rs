macro_rules! retain_adt_literal_usages {
    () => {
        # [doc = " Filter out all non-literal usages for adt-defs"] fn retain_adt_literal_usages (usages : & mut UsageSearchResult , def : Definition , sema : & Semantics < '_ , RootDatabase > ,) { let refs = usages . references . values_mut () ; match def { Definition :: Adt (hir :: Adt :: Enum (enum_)) => { refs . for_each (| it | { it . retain (| reference | { reference . name . as_name_ref () . is_some_and (| name_ref | is_enum_lit_name_ref (sema , enum_ , name_ref)) }) }) ; usages . references . retain (| _ , it | ! it . is_empty ()) ; } Definition :: Adt (_) | Definition :: Variant (_) => { refs . for_each (| it | { it . retain (| reference | reference . name . as_name_ref () . is_some_and (is_lit_name_ref)) }) ; usages . references . retain (| _ , it | ! it . is_empty ()) ; } _ => { } } }
    };
}

retain_adt_literal_usages!()
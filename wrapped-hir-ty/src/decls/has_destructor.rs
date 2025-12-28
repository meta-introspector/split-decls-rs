macro_rules! deps {
    () => {
        HirDatabase!();
        TraitImpls!();
    };
}

macro_rules! has_destructor {
    () => {
        deps!();
        fn has_destructor (db : & dyn HirDatabase , adt : AdtId) -> bool { let module = match adt { AdtId :: EnumId (id) => db . lookup_intern_enum (id) . container , AdtId :: StructId (id) => db . lookup_intern_struct (id) . container , AdtId :: UnionId (id) => db . lookup_intern_union (id) . container , } ; let Some (drop_trait) = LangItem :: Drop . resolve_trait (db , module . krate ()) else { return false ; } ; let impls = match module . containing_block () { Some (block) => match TraitImpls :: for_block (db , block) { Some (it) => & * * it , None => return false , } , None => TraitImpls :: for_crate (db , module . krate ()) , } ; ! impls . for_trait_and_self_ty (drop_trait , & SimplifiedType :: Adt (adt . into ())) . is_empty () }
    };
}

has_destructor!()
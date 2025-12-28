macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! field_ty {
    () => {
        deps!();
        fn field_ty < 'a > (db : & 'a dyn HirDatabase , def : hir_def :: VariantId , fd : LocalFieldId , args : & GenericArgs < 'a > ,) -> Ty < 'a > { db . field_types (def) [fd] . instantiate (DbInterner :: new_with (db , None , None) , args) }
    };
}

field_ty!();
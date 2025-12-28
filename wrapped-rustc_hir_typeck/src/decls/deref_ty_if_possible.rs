macro_rules! deref_ty_if_possible {
    () => {
        # [doc = " Dereferences a single level of immutable referencing."] fn deref_ty_if_possible (ty : Ty < '_ >) -> Ty < '_ > { match ty . kind () { ty :: Ref (_ , ty , hir :: Mutability :: Not) => * ty , _ => ty , } }
    };
}

deref_ty_if_possible!()
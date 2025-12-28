macro_rules! deps {
    () => {
        RootDatabase!();
        Definition!();
    };
}

macro_rules! def_to_ty {
    () => {
        deps!();
        fn def_to_ty < 'db > (sema : & Semantics < 'db , RootDatabase > , def : & Definition) -> Option < hir :: Type < 'db > > { match def { Definition :: Adt (adt) => Some (adt . ty (sema . db)) , Definition :: TypeAlias (it) => Some (it . ty (sema . db)) , Definition :: BuiltinType (it) => Some (it . ty (sema . db)) , Definition :: SelfType (it) => Some (it . self_ty (sema . db)) , _ => None , } }
    };
}

def_to_ty!();
macro_rules! deps {
    () => {
        UnqualifiedName!();
        IsCtorDtorConversion!();
        OperatorName!();
        SubstitutionTable!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl IsCtorDtorConversion for UnqualifiedName { fn is_ctor_dtor_conversion (& self , _ : & SubstitutionTable) -> bool { match * self { UnqualifiedName :: CtorDtor (..) | UnqualifiedName :: Operator (OperatorName :: Conversion (_) , _) => true , UnqualifiedName :: Operator (..) | UnqualifiedName :: Source (..) | UnqualifiedName :: LocalSourceName (..) | UnqualifiedName :: UnnamedType (..) | UnqualifiedName :: ClosureType (..) => false , } } }
    };
}

impl_115!()
macro_rules! deps {
    () => {
        LowerTypeRelativePathMode!();
        PermitVariants!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl LowerTypeRelativePathMode { fn assoc_tag (self) -> ty :: AssocTag { match self { Self :: Type (_) => ty :: AssocTag :: Type , Self :: Const => ty :: AssocTag :: Const , } } fn def_kind (self) -> DefKind { match self { Self :: Type (_) => DefKind :: AssocTy , Self :: Const => DefKind :: AssocConst , } } fn permit_variants (self) -> PermitVariants { match self { Self :: Type (permit_variants) => permit_variants , Self :: Const => PermitVariants :: No , } } }
    };
}

impl_472!()
macro_rules! deps {
    () => {
        TyKind!();
        InferArg!();
        Ty!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl InferArg { pub fn to_ty (& self) -> Ty < 'static > { Ty { kind : TyKind :: Infer (()) , span : self . span , hir_id : self . hir_id } } }
    };
}

impl_125!()
macro_rules! deps {
    () => {
        MutTy!();
        AmbigArg!();
        Ty!();
        TyKind!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < 'hir > Ty < 'hir , AmbigArg > { pub fn peel_refs (& self) -> & Ty < 'hir > { let mut final_ty = self . as_unambig_ty () ; while let TyKind :: Ref (_ , MutTy { ty , .. }) = & final_ty . kind { final_ty = ty ; } final_ty } }
    };
}

impl_266!()
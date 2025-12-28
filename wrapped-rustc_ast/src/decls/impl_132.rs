macro_rules! deps {
    () => {
        Ty!();
        TyKind!();
        MutTy!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl Ty { pub fn peel_refs (& self) -> & Self { let mut final_ty = self ; while let TyKind :: Ref (_ , MutTy { ty , .. }) | TyKind :: Ptr (MutTy { ty , .. }) = & final_ty . kind { final_ty = ty ; } final_ty } pub fn is_maybe_parenthesised_infer (& self) -> bool { match & self . kind { TyKind :: Infer => true , TyKind :: Paren (inner) => inner . is_maybe_parenthesised_infer () , _ => false , } } }
    };
}

impl_132!();
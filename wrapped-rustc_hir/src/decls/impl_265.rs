macro_rules! deps {
    () => {
        Ty!();
        AmbigArg!();
        TyKind!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < 'hir > Ty < 'hir > { # [doc = " Converts a `Ty` in an unambiguous position to one in an ambiguous position. This is"] # [doc = " fallible as the [`TyKind::Infer`] variant is not present in ambiguous positions."] # [doc = ""] # [doc = " Functions accepting ambiguous types will not handle the [`TyKind::Infer`] variant, if"] # [doc = " infer types are relevant to you then care should be taken to handle them separately."] pub fn try_as_ambig_ty (& self) -> Option < & Ty < 'hir , AmbigArg > > { if let TyKind :: Infer (()) = self . kind { return None ; } let ptr = self as * const Ty < 'hir > as * const Ty < 'hir , AmbigArg > ; Some (unsafe { & * ptr }) } }
    };
}

impl_265!();
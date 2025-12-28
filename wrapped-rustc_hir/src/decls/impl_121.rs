macro_rules! deps {
    () => {
        AmbigArg!();
        ConstArgKind!();
        ConstArg!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'hir > ConstArg < 'hir > { # [doc = " Converts a `ConstArg` in an unambiguous position to one in an ambiguous position. This is"] # [doc = " fallible as the [`ConstArgKind::Infer`] variant is not present in ambiguous positions."] # [doc = ""] # [doc = " Functions accepting ambiguous consts will not handle the [`ConstArgKind::Infer`] variant, if"] # [doc = " infer consts are relevant to you then care should be taken to handle them separately."] pub fn try_as_ambig_ct (& self) -> Option < & ConstArg < 'hir , AmbigArg > > { if let ConstArgKind :: Infer (_ , ()) = self . kind { return None ; } let ptr = self as * const ConstArg < 'hir > as * const ConstArg < 'hir , AmbigArg > ; Some (unsafe { & * ptr }) } }
    };
}

impl_121!();
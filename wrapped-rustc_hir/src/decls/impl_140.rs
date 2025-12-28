macro_rules! deps {
    () => {
        Lifetime!();
        LifetimeParamKind!();
        GenericParamKind!();
        GenericParam!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'hir > GenericParam < 'hir > { # [doc = " Synthetic type-parameters are inserted after normal ones."] # [doc = " In order for normal parameters to be able to refer to synthetic ones,"] # [doc = " scans them first."] pub fn is_impl_trait (& self) -> bool { matches ! (self . kind , GenericParamKind :: Type { synthetic : true , .. }) } # [doc = " This can happen for `async fn`, e.g. `async fn f<'_>(&'_ self)`."] # [doc = ""] # [doc = " See `lifetime_to_generic_param` in `rustc_ast_lowering` for more information."] pub fn is_elided_lifetime (& self) -> bool { matches ! (self . kind , GenericParamKind :: Lifetime { kind : LifetimeParamKind :: Elided (_) }) } }
    };
}

impl_140!()
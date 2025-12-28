macro_rules! AsCoercionSite {
    () => {
        # [doc = " Something that can be converted into an expression to which we can"] # [doc = " apply a coercion."] pub (crate) trait AsCoercionSite { fn as_coercion_site (& self) -> & hir :: Expr < '_ > ; }
    };
}

AsCoercionSite!();
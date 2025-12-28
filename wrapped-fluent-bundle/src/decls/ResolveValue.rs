macro_rules! deps {
    () => {
        FluentValue!();
        Scope!();
        FluentResource!();
        MemoizerKind!();
    };
}

macro_rules! ResolveValue {
    () => {
        deps!();
        # [doc = " Resolves an AST node to a [`FluentValue`]."] pub (crate) trait ResolveValue < 'bundle > { # [doc = " Resolves an AST node to a [`FluentValue`]."] fn resolve < 'ast , 'args , 'errors , R , M > (& 'ast self , scope : & mut Scope < 'bundle , 'ast , 'args , 'errors , R , M > ,) -> FluentValue < 'bundle > where R : Borrow < FluentResource > , M : MemoizerKind ; }
    };
}

ResolveValue!()
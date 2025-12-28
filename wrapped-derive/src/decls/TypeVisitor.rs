macro_rules! TypeVisitor {
    () => {
        struct TypeVisitor < 'a > { # [doc = " The type parameters in scope"] typarams : & 'a HashMap < Ident , Option < Ident > > , # [doc = " Whether we found a type parameter"] found_typarams : bool , # [doc = " Whether we found a non-'static lifetime parameter"] found_lifetimes : bool , }
    };
}

TypeVisitor!();
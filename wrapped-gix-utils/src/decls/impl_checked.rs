macro_rules! impl_checked {
    () => {
        macro_rules ! impl_checked { ($ f : ident) => { fn $ f (self , rhs : Self) -> Option < Self > { Self ::$ f (self , rhs) } } ; }
    };
}

impl_checked!();
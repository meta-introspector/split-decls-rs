macro_rules! deps {
    () => {
        GenericBound!();
        Trait!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl GenericBound { pub fn span (& self) -> Span { match self { GenericBound :: Trait (t , ..) => t . span , GenericBound :: Outlives (l) => l . ident . span , GenericBound :: Use (_ , span) => * span , } } }
    };
}

impl_30!()
macro_rules! deps {
    () => {
        GenericBound!();
        TraitRef!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl GenericBound < '_ > { pub fn trait_ref (& self) -> Option < & TraitRef < '_ > > { match self { GenericBound :: Trait (data) => Some (& data . trait_ref) , _ => None , } } pub fn span (& self) -> Span { match self { GenericBound :: Trait (t , ..) => t . span , GenericBound :: Outlives (l) => l . ident . span , GenericBound :: Use (_ , span) => * span , } } }
    };
}

impl_134!();
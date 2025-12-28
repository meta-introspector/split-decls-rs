macro_rules! deps {
    () => {
        BoundKind!();
        Impl!();
        Trait!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl BoundKind { pub fn descr (self) -> & 'static str { match self { BoundKind :: Bound => "bounds" , BoundKind :: Impl => "`impl Trait`" , BoundKind :: TraitObject => "`dyn` trait object bounds" , BoundKind :: SuperTraits => "supertrait bounds" , } } }
    };
}

impl_466!();
macro_rules! deps {
    () => {
        ParamName!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl ParamName { pub fn ident (& self) -> Ident { match * self { ParamName :: Plain (ident) | ParamName :: Error (ident) => ident , ParamName :: Fresh => Ident :: with_dummy_span (kw :: UnderscoreLifetime) , } } }
    };
}

impl_109!()
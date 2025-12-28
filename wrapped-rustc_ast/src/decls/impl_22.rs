macro_rules! deps {
    () => {
        AngleBracketedArg!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl AngleBracketedArg { pub fn span (& self) -> Span { match self { AngleBracketedArg :: Arg (arg) => arg . span () , AngleBracketedArg :: Constraint (constraint) => constraint . span , } } }
    };
}

impl_22!();
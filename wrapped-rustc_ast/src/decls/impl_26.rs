macro_rules! deps {
    () => {
        Type!();
        ParenthesizedArgs!();
        AngleBracketedArgs!();
        AngleBracketedArg!();
        GenericArg!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ParenthesizedArgs { pub fn as_angle_bracketed_args (& self) -> AngleBracketedArgs { let args = self . inputs . iter () . cloned () . map (| input | AngleBracketedArg :: Arg (GenericArg :: Type (input))) . collect () ; AngleBracketedArgs { span : self . inputs_span , args } } }
    };
}

impl_26!()
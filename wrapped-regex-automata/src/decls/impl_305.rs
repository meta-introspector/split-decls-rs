macro_rules! deps {
    () => {
        BuildErrorKind!();
        NFA!();
        BuildError!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . kind { BuildErrorKind :: Syntax { pid , .. } => { write ! (f , "error parsing pattern {}" , pid . as_usize ()) } BuildErrorKind :: NFA (_) => write ! (f , "error building NFA") , } } }
    };
}

impl_305!();
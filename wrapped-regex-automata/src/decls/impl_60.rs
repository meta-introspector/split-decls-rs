macro_rules! deps {
    () => {
        BuildErrorKind!();
        BuildError!();
        NFA!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [cfg (all (feature = "std" , feature = "dfa-build"))] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind () { BuildErrorKind :: NFA (ref err) => Some (err) , _ => None , } } }
    };
}

impl_60!()
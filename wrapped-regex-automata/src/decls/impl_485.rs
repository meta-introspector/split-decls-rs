macro_rules! deps {
    () => {
        BuildErrorKind!();
        Captures!();
        BuildError!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind () { # [cfg (feature = "syntax")] BuildErrorKind :: Syntax (ref err) => Some (err) , BuildErrorKind :: Captures (ref err) => Some (err) , _ => None , } } }
    };
}

impl_485!();
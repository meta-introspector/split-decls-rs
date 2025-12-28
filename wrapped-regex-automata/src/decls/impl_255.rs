macro_rules! deps {
    () => {
        BuildError!();
        BuildErrorKind!();
        NFA!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { BuildErrorKind :: NFA (ref err) => Some (err) , _ => None , } } }
    };
}

impl_255!();
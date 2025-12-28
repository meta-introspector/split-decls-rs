macro_rules! deps {
    () => {
        NFA!();
        BuildErrorKind!();
        BuildError!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { BuildErrorKind :: Syntax { ref err , .. } => Some (err) , BuildErrorKind :: NFA (ref err) => Some (err) , } } }
    };
}

impl_304!()
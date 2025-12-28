macro_rules! deps {
    () => {
        NFA!();
        BuildError!();
        BuildErrorKind!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { use self :: BuildErrorKind :: * ; match self . kind { NFA (ref err) => Some (err) , Word (ref err) => Some (err) , _ => None , } } }
    };
}

impl_97!()
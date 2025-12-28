macro_rules! deps {
    () => {
        PrefixKind!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl PrefixKind { # [inline] fn path_kind (self) -> PathKind { match self { PrefixKind :: BySelf => PathKind :: SELF , PrefixKind :: Plain => PathKind :: Plain , PrefixKind :: ByCrate => PathKind :: Crate , } } }
    };
}

impl_401!();
macro_rules! deps {
    () => {
        Deprecation!();
    };
}

macro_rules! impl_1035 {
    () => {
        deps!();
        impl Deprecation { # [inline] pub fn is_deprecated (& self) -> bool { matches ! (self , Deprecation :: Deprecated { .. }) } # [inline] pub fn reason (& self) -> Option < & str > { match self { Deprecation :: NoDeprecated => None , Deprecation :: Deprecated { reason } => reason . as_deref () , } } }
    };
}

impl_1035!();
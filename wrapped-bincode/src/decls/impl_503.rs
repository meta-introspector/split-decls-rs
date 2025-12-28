macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl core :: error :: Error for DecodeError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: Utf8 { inner } => Some (inner) , _ => None , } } }
    };
}

impl_503!();
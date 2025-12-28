macro_rules! deps {
    () => {
        BoundPolarity!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl BoundPolarity { pub fn as_str (self) -> & 'static str { match self { Self :: Positive => "" , Self :: Negative (_) => "!" , Self :: Maybe (_) => "?" , } } }
    };
}

impl_172!()
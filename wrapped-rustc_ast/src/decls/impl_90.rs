macro_rules! deps {
    () => {
        RangeLimits!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl RangeLimits { pub fn as_str (& self) -> & 'static str { match self { RangeLimits :: HalfOpen => ".." , RangeLimits :: Closed => "..=" , } } }
    };
}

impl_90!()
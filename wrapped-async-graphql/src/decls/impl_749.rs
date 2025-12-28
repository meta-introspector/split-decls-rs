macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl < T : Into < Value > > From < T > for Any { fn from (value : T) -> Any { Any (value . into ()) } }
    };
}

impl_749!();
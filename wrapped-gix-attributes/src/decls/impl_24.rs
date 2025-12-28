macro_rules! deps {
    () => {
        StateRef!();
        State!();
        Value!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a > From < StateRef < 'a > > for State { fn from (s : StateRef < 'a >) -> Self { match s { StateRef :: Value (v) => State :: Value (v . into ()) , StateRef :: Set => State :: Set , StateRef :: Unset => State :: Unset , StateRef :: Unspecified => State :: Unspecified , } } }
    };
}

impl_24!()
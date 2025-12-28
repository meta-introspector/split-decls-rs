macro_rules! deps {
    () => {
        StateRef!();
        Value!();
        State!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a > State { # [doc = " Turn ourselves into our ref-type."] pub fn as_ref (& 'a self) -> StateRef < 'a > { match self { State :: Value (v) => StateRef :: Value (v . as_ref ()) , State :: Set => StateRef :: Set , State :: Unset => StateRef :: Unset , State :: Unspecified => StateRef :: Unspecified , } } }
    };
}

impl_23!()
macro_rules! deps {
    () => {
        Float!();
        Number!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        impl Number { # [cfg (not (feature = "arbitrary_precision"))] # [cold] pub (crate) fn unexpected (& self) -> Unexpected { match self . n { N :: PosInt (u) => Unexpected :: Unsigned (u) , N :: NegInt (i) => Unexpected :: Signed (i) , N :: Float (f) => Unexpected :: Float (f) , } } # [cfg (feature = "arbitrary_precision")] # [cold] pub (crate) fn unexpected (& self) -> Unexpected { Unexpected :: Other ("number") } }
    };
}

impl_565!()
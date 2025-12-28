macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl Sub for Fe { type Output = Fe ; fn sub (self , _rhs : Fe) -> Fe { let Fe (f) = self ; let Fe (g) = _rhs ; let mut h = Fe :: default () ; fiat_25519_sub (& mut h . 0 , & f , & g) ; h . carry () } }
    };
}

impl_43!()
macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Add for Fe { type Output = Fe ; fn add (self , _rhs : Fe) -> Fe { let Fe (f) = self ; let Fe (g) = _rhs ; let mut h = Fe :: default () ; fiat_25519_add (& mut h . 0 , & f , & g) ; h } }
    };
}

impl_42!()
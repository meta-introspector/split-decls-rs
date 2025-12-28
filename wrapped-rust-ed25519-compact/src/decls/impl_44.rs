macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Mul for Fe { type Output = Fe ; fn mul (self , _rhs : Fe) -> Fe { let Fe (f) = self ; let Fe (g) = _rhs ; let mut h = Fe :: default () ; fiat_25519_carry_mul (& mut h . 0 , & f , & g) ; h } }
    };
}

impl_44!();
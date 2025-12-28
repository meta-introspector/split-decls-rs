macro_rules! deps {
    () => {
        U64x4!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl core :: ops :: BitXorAssign for U64x4 { fn bitxor_assign (& mut self , _rhs : Self) { self . 0 ^= _rhs . 0 ; self . 1 ^= _rhs . 1 ; self . 2 ^= _rhs . 2 ; self . 3 ^= _rhs . 3 ; } }
    };
}

impl_54!()
macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'a , T , const N : usize > IndexConst < & 'a [T ; N] > { # [inline (always)] # [allow (unused)] const fn index (self , i : usize) -> & 'a T { & self . 0 [i] } }
    };
}

impl_141!()
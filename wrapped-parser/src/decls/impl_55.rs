macro_rules! deps {
    () => {
        Input!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Input { fn bit_index (& self , n : usize) -> (usize , usize) { let idx = n / (bits :: BITS as usize) ; let b_idx = n % (bits :: BITS as usize) ; (idx , b_idx) } fn len (& self) -> usize { self . kind . len () } }
    };
}

impl_55!();
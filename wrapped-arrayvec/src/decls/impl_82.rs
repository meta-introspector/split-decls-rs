macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T , const CAP : usize > Default for ArrayVec < T , CAP > { # [doc = " Return an empty array"] fn default () -> ArrayVec < T , CAP > { ArrayVec :: new () } }
    };
}

impl_82!();
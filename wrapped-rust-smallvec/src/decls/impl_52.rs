macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T , const N : usize > core :: ops :: DerefMut for SmallVec < T , N > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
    };
}

impl_52!()
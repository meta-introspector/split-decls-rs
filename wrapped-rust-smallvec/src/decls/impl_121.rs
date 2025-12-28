macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T , const N : usize > core :: ops :: Deref for SmallVec < T , N > { type Target = [T] ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
    };
}

impl_121!();
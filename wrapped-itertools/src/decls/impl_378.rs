macro_rules! deps {
    () => {
        ArrayBuilder!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < T , const N : usize > AsMut < [T] > for ArrayBuilder < T , N > { fn as_mut (& mut self) -> & mut [T] { let valid = & mut self . arr [.. self . len] ; unsafe { slice_assume_init_mut (valid) } } }
    };
}

impl_378!();
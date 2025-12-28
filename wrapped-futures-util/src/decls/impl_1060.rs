macro_rules! impl_1060 {
    () => {
        impl < R : AsyncRead > BufReader < R > { # [doc = " Creates a new `BufReader` with a default buffer capacity. The default is currently 8 KB,"] # [doc = " but may change in the future."] pub fn new (inner : R) -> Self { Self :: with_capacity (DEFAULT_BUF_SIZE , inner) } # [doc = " Creates a new `BufReader` with the specified buffer capacity."] pub fn with_capacity (capacity : usize , inner : R) -> Self { let buffer = vec ! [0 ; capacity] ; Self { inner , buffer : buffer . into_boxed_slice () , pos : 0 , cap : 0 } } }
    };
}

impl_1060!();
macro_rules! macro_257 {
    () => {
        zerocopy :: cryptocorrosion_derive_traits ! { # [repr (C)] # [derive (Clone , Copy)] pub union vec128_storage { d : [u32 ; 4] , q : [u64 ; 2] , } }
    };
}

macro_257!();
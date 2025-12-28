macro_rules! macro_32 {
    () => {
        zerocopy :: cryptocorrosion_derive_traits ! { # [repr (transparent)] # [derive (Copy , Clone , Default)] # [allow (non_camel_case_types)] pub struct x4 < W > (pub [W ; 4]) ; }
    };
}

macro_32!();
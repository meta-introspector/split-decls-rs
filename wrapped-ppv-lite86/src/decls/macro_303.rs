macro_rules! macro_303 {
    () => {
        zerocopy :: cryptocorrosion_derive_traits ! { # [repr (transparent)] # [derive (Copy , Clone , Debug , PartialEq)] pub struct u64x2_generic ([u64 ; 2]) ; }
    };
}

macro_303!();
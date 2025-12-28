macro_rules! impl_25 {
    () => {
        impl i256 { # [doc = " Reinterpret as an unsigned integer"] # [cfg (any (test , feature = "unstable-public-internals"))] pub fn unsigned (self) -> u256 { u256 { lo : self . lo , hi : self . hi as u128 , } } }
    };
}

impl_25!()
macro_rules! impl_23 {
    () => {
        impl u256 { # [cfg (any (test , feature = "unstable-public-internals"))] pub const MAX : Self = Self { lo : u128 :: MAX , hi : u128 :: MAX , } ; # [doc = " Reinterpret as a signed integer"] pub fn signed (self) -> i256 { i256 { lo : self . lo , hi : self . hi as i128 , } } }
    };
}

impl_23!();
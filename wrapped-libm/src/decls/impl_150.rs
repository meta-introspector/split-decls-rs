macro_rules! deps {
    () => {
        OtherSign!();
        MinInt!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl MinInt for u256 { type OtherSign = i256 ; type Unsigned = u256 ; const SIGNED : bool = false ; const BITS : u32 = 256 ; const ZERO : Self = Self { lo : 0 , hi : 0 } ; const ONE : Self = Self { lo : 1 , hi : 0 } ; const MIN : Self = Self { lo : 0 , hi : 0 } ; const MAX : Self = Self { lo : u128 :: MAX , hi : u128 :: MAX , } ; }
    };
}

impl_150!()
macro_rules! deps {
    () => {
        MinInt!();
        OtherSign!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl MinInt for i256 { type OtherSign = u256 ; type Unsigned = u256 ; const SIGNED : bool = true ; const BITS : u32 = 256 ; const ZERO : Self = Self { lo : 0 , hi : 0 } ; const ONE : Self = Self { lo : 1 , hi : 0 } ; const MIN : Self = Self { lo : u128 :: MIN , hi : i128 :: MIN , } ; const MAX : Self = Self { lo : u128 :: MAX , hi : i128 :: MAX , } ; }
    };
}

impl_151!()
macro_rules! deps {
    () => {
        ReaderOffset!();
        Result!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl ReaderOffset for u64 { # [inline] fn from_u8 (offset : u8) -> Self { u64 :: from (offset) } # [inline] fn from_u16 (offset : u16) -> Self { u64 :: from (offset) } # [inline] fn from_i16 (offset : i16) -> Self { offset as u64 } # [inline] fn from_u32 (offset : u32) -> Self { u64 :: from (offset) } # [inline] fn from_u64 (offset : u64) -> Result < Self > { Ok (offset) } # [inline] fn into_u64 (self) -> u64 { self } # [inline] fn wrapping_add (self , other : Self) -> Self { self . wrapping_add (other) } # [inline] fn checked_sub (self , other : Self) -> Option < Self > { self . checked_sub (other) } }
    };
}

impl_318!();
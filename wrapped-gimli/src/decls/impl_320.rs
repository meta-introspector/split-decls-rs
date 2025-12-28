macro_rules! deps {
    () => {
        Result!();
        ReaderOffset!();
        Error!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl ReaderOffset for usize { # [inline] fn from_u8 (offset : u8) -> Self { offset as usize } # [inline] fn from_u16 (offset : u16) -> Self { offset as usize } # [inline] fn from_i16 (offset : i16) -> Self { offset as usize } # [inline] fn from_u32 (offset : u32) -> Self { offset as usize } # [inline] fn from_u64 (offset64 : u64) -> Result < Self > { let offset = offset64 as usize ; if offset as u64 == offset64 { Ok (offset) } else { Err (Error :: UnsupportedOffset) } } # [inline] fn into_u64 (self) -> u64 { self as u64 } # [inline] fn wrapping_add (self , other : Self) -> Self { self . wrapping_add (other) } # [inline] fn checked_sub (self , other : Self) -> Option < Self > { self . checked_sub (other) } }
    };
}

impl_320!()
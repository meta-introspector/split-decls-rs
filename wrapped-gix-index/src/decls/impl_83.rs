macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl From < gix_object :: tree :: EntryMode > for Mode { fn from (value : gix_object :: tree :: EntryMode) -> Self { let value : u16 = value . value () ; Self :: from_bits_truncate (u32 :: from (value)) } }
    };
}

impl_83!()
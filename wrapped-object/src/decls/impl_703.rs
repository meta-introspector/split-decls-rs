macro_rules! deps {
    () => {
        ImageThunkData64!();
        ImageThunkData!();
    };
}

macro_rules! impl_703 {
    () => {
        deps!();
        impl ImageThunkData for pe :: ImageThunkData64 { fn raw (self) -> u64 { self . 0 . get (LE) } fn is_ordinal (self) -> bool { self . 0 . get (LE) & pe :: IMAGE_ORDINAL_FLAG64 != 0 } fn ordinal (self) -> u16 { self . 0 . get (LE) as u16 } fn address (self) -> u32 { self . 0 . get (LE) as u32 & 0x7fff_ffff } }
    };
}

impl_703!();
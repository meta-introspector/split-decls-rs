macro_rules! deps {
    () => {
        ImageThunkData!();
        ImageThunkData32!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl ImageThunkData for pe :: ImageThunkData32 { fn raw (self) -> u64 { self . 0 . get (LE) . into () } fn is_ordinal (self) -> bool { self . 0 . get (LE) & pe :: IMAGE_ORDINAL_FLAG32 != 0 } fn ordinal (self) -> u16 { self . 0 . get (LE) as u16 } fn address (self) -> u32 { self . 0 . get (LE) & 0x7fff_ffff } }
    };
}

impl_704!()
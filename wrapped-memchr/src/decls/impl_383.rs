macro_rules! deps {
    () => {
        SensibleMoveMask!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl SensibleMoveMask { # [doc = " Get the mask in a form suitable for computing offsets."] # [doc = ""] # [doc = " Basically, this normalizes to little endian. On big endian, this swaps"] # [doc = " the bytes."] # [inline (always)] fn get_for_offset (self) -> u32 { # [cfg (target_endian = "big")] { self . 0 . swap_bytes () } # [cfg (target_endian = "little")] { self . 0 } } }
    };
}

impl_383!()
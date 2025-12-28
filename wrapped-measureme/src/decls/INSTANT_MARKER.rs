macro_rules! INSTANT_MARKER {
    () => {
        # [doc = " `RawEvents` that have a payload 2 value with this value are instant events."] const INSTANT_MARKER : u64 = 0xFFFF_FFFF_FFFF ;
    };
}

INSTANT_MARKER!();
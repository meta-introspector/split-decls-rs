macro_rules! deps {
    () => {
        Position!();
        ExtraEdge!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ExtraEdge { pub fn from_raw (raw : u32) -> Self { if raw & LAST_EXTENDED_EDGE_MASK != 0 { Self :: Last (Position (raw & ! LAST_EXTENDED_EDGE_MASK)) } else { Self :: Internal (Position (raw)) } } }
    };
}

impl_25!()
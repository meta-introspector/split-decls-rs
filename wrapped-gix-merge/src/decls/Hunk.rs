macro_rules! deps {
    () => {
        Side!();
    };
}

macro_rules! Hunk {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct Hunk { pub before : Range < u32 > , pub after : Range < u32 > , pub side : Side , }
    };
}

Hunk!();
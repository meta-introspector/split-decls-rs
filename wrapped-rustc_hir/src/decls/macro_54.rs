macro_rules! deps {
    () => {
        MacroKinds!();
    };
}

macro_rules! macro_54 {
    () => {
        deps!();
        bitflags :: bitflags ! { impl MacroKinds : u8 { const BANG = 1 << 0 ; const ATTR = 1 << 1 ; const DERIVE = 1 << 2 ; } }
    };
}

macro_54!();
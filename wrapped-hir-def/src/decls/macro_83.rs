macro_rules! macro_83 {
    () => {
        bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct MacroRulesLocFlags : u8 { const ALLOW_INTERNAL_UNSAFE = 1 << 0 ; const LOCAL_INNER = 1 << 1 ; } }
    };
}

macro_83!()
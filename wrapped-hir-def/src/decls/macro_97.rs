macro_rules! macro_97 {
    () => {
        bitflags :: bitflags ! { # [derive (Debug , Clone , Copy , Eq , PartialEq , Default)] pub struct ImplFlags : u8 { const NEGATIVE = 1 << 1 ; const DEFAULT = 1 << 2 ; const UNSAFE = 1 << 3 ; } }
    };
}

macro_97!();
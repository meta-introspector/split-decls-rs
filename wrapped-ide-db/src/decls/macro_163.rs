macro_rules! macro_163 {
    () => {
        bitflags :: bitflags ! { # [derive (Copy , Clone , Default , PartialEq , Eq , Hash , Debug)] pub struct ReferenceCategory : u8 { const WRITE = 1 << 0 ; const READ = 1 << 1 ; const IMPORT = 1 << 2 ; const TEST = 1 << 3 ; } }
    };
}

macro_163!();
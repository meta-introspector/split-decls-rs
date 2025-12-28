macro_rules! macro_73 {
    () => {
        bitflags ! { # [doc = " Describes only the presence/absence of each namespace, without its value."] # [derive (Debug , PartialEq , Eq)] pub (crate) struct NsAvailability : u32 { const TYPES = 1 << 0 ; const VALUES = 1 << 1 ; const MACROS = 1 << 2 ; } }
    };
}

macro_73!()
macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        __impl_internal_bitflags ! { Field0 : u32 , Flags { # [doc = ""] # [doc = " This flag has the value `0b00000001`."] const A = 0b00000001 ; # [doc = " Field `B`."] # [doc = ""] # [doc = " This flag has the value `0b00000010`."] const B = 0b00000010 ; # [doc = " Field `C`."] # [doc = ""] # [doc = " This flag has the value `0b00000100`."] const C = 0b00000100 ; const ABC = Self :: A . bits () | Self :: B . bits () | Self :: C . bits () ; } }
    };
}

macro_73!();
// Generated macro for macro_780 (macro)
macro_rules! Depcrate_nummacro_780 {
() => {
// Module: crate::num
// Provides: {"macro_780"}
// Dependencies: {}
bitflags ! { # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub (crate) struct FloatTypes : u32 { const POSITIVE = 0b0000_0001 ; const NEGATIVE = 0b0000_0010 ; const NORMAL = 0b0000_0100 ; const SUBNORMAL = 0b0000_1000 ; const ZERO = 0b0001_0000 ; const INFINITE = 0b0010_0000 ; const QUIET_NAN = 0b0100_0000 ; const SIGNALING_NAN = 0b1000_0000 ; const ANY = Self :: POSITIVE . bits () | Self :: NEGATIVE . bits () | Self :: NORMAL . bits () | Self :: SUBNORMAL . bits () | Self :: ZERO . bits () | Self :: INFINITE . bits () | Self :: QUIET_NAN . bits () ; } }
};
}

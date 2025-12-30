// Generated macro for macro_172 (macro)
macro_rules! Depcrate_appkitmacro_172 {
() => {
// Module: crate::appkit
// Provides: {"macro_172"}
// Dependencies: {}
bitflags ! { # [derive (Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct NSTouchPhase : NSUInteger { const NSTouchPhaseBegan = 1 << 0 ; const NSTouchPhaseMoved = 1 << 1 ; const NSTouchPhaseStationary = 1 << 2 ; const NSTouchPhaseEnded = 1 << 3 ; const NSTouchPhaseCancelled = 1 << 4 ; const NSTouchPhaseTouching = NSTouchPhase :: NSTouchPhaseBegan . bits () | NSTouchPhase :: NSTouchPhaseMoved . bits () | NSTouchPhase :: NSTouchPhaseStationary . bits () ; const NSTouchPhaseAny = ! 0 ; } }
};
}

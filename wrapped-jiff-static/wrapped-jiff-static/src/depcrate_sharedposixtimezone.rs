// Generated macro for PosixTimeZone (struct)
macro_rules! Depcrate_sharedPosixTimeZone {
() => {
// Module: crate::shared
// Provides: {"PosixTimeZone"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct PosixTimeZone < ABBREV > { pub std_abbrev : ABBREV , pub std_offset : PosixOffset , pub dst : Option < PosixDst < ABBREV > > , }
};
}

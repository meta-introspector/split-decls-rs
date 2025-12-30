// Generated macro for Time (struct)
macro_rules! Depcrate_systemTime {
() => {
// Module: crate::system
// Provides: {"Time"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug , Default)] pub struct Time { pub year : u16 , pub month : u8 , pub day : u8 , pub hour : u8 , pub minute : u8 , pub second : u8 , pub pad1 : u8 , pub nanosecond : u32 , pub timezone : i16 , pub daylight : u8 , pub pad2 : u8 , }
};
}

// Generated macro for DestinationShorthand (enum)
macro_rules! Depcrate_apicDestinationShorthand {
() => {
// Module: crate::apic
// Provides: {"DestinationShorthand"}
// Dependencies: {}
# [doc = " IPI Destination Shorthand"] # [derive (Debug , Eq , PartialEq)] # [repr (u64)] pub enum DestinationShorthand { NoShorthand = 0b00 , Myself = 0b01 , AllIncludingSelf = 0b10 , AllExcludingSelf = 0b11 , }
};
}

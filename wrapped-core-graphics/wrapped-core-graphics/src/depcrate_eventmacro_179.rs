// Generated macro for macro_179 (macro)
macro_rules! Depcrate_eventmacro_179 {
() => {
// Module: crate::event
// Provides: {"macro_179"}
// Dependencies: {}
bitflags ! { # [doc = " Flags for events"] # [doc = ""] # [doc = " [Ref](http://opensource.apple.com/source/IOHIDFamily/IOHIDFamily-700/IOHIDSystem/IOKit/hidsystem/IOLLEvent.h)"] # [repr (C)] # [derive (Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct CGEventFlags : u64 { const CGEventFlagNull = 0 ; const CGEventFlagAlphaShift = 0x00010000 ; const CGEventFlagShift = 0x00020000 ; const CGEventFlagControl = 0x00040000 ; const CGEventFlagAlternate = 0x00080000 ; const CGEventFlagCommand = 0x00100000 ; const CGEventFlagHelp = 0x00400000 ; const CGEventFlagSecondaryFn = 0x00800000 ; const CGEventFlagNumericPad = 0x00200000 ; const CGEventFlagNonCoalesced = 0x00000100 ; } }
};
}

// Generated macro for CAClockMessage (struct)
macro_rules! Depcrate_generatedCAClockMessage {
() => {
// Module: crate::generated
// Provides: {"CAClockMessage"}
// Dependencies: {}
# [doc = " The messages sent to a CAClockListenerProc to notify the client of"] # [doc = " changes to the clock's state."] # [doc = ""] # [doc = ""] # [doc = " A new start time was set or received from an external sync source."] # [doc = ""] # [doc = " The clock's time has started moving."] # [doc = ""] # [doc = " The clock's time has stopped moving."] # [doc = ""] # [doc = " The client has called CAClockArm()."] # [doc = ""] # [doc = " The client has called CAClockDisarm()."] # [doc = ""] # [doc = " A clock property has been changed."] # [doc = ""] # [doc = " The clock is receiving SMPTE (MTC) messages in a SMPTE format that does not"] # [doc = " match the clock's SMPTE format."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caclockmessage?language=objc)"] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct CAClockMessage (pub u32) ;
};
}

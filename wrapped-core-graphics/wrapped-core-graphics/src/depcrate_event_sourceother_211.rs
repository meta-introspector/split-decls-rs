// Generated macro for other_211 (other)
macro_rules! Depcrate_event_sourceother_211 {
() => {
// Module: crate::event_source
// Provides: {"other_211"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { # [doc = " Return the type identifier for the opaque type [`CGEventSourceRef`]."] # [doc = ""] # [doc = " [`CGEventSourceRef`]: crate::sys::CGEventSourceRef"] fn CGEventSourceGetTypeID () -> CFTypeID ; # [doc = " Return a Quartz event source created with a specified source state."] fn CGEventSourceCreate (stateID : CGEventSourceStateID) -> crate :: sys :: CGEventSourceRef ; }
};
}

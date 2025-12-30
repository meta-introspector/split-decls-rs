// Generated macro for impl_199 (impl)
macro_rules! Depcrate_eventimpl_199 {
() => {
// Module: crate::event
// Provides: {"impl_199"}
// Dependencies: {}
impl CGEventTap < 'static > { pub fn new < F : Fn (CGEventTapProxy , CGEventType , & CGEvent) -> CallbackResult + Send + 'static > (tap : CGEventTapLocation , place : CGEventTapPlacement , options : CGEventTapOptions , events_of_interest : std :: vec :: Vec < CGEventType > , callback : F ,) -> Result < Self , () > { unsafe { Self :: new_unchecked (tap , place , options , events_of_interest , callback) } } }
};
}

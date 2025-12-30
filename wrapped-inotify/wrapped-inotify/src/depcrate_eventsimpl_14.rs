// Generated macro for impl_14 (impl)
macro_rules! Depcrate_eventsimpl_14 {
() => {
// Module: crate::events
// Provides: {"impl_14"}
// Dependencies: {}
impl EventMask { # [doc = " Parse this event mask into a ParsedEventMask"] pub fn parse (self) -> Result < ParsedEventMask , EventMaskParseError > { self . try_into () } # [doc = " Wrapper around [`Self::from_bits_retain`] for backwards compatibility"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is not actually unsafe. It is just a wrapper around the"] # [doc = " safe [`Self::from_bits_retain`]."] # [deprecated = "Use the safe `from_bits_retain` method instead"] pub unsafe fn from_bits_unchecked (bits : u32) -> Self { Self :: from_bits_retain (bits) } }
};
}

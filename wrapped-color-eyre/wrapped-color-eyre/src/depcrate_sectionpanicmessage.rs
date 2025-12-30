// Generated macro for PanicMessage (trait)
macro_rules! Depcrate_sectionPanicMessage {
() => {
// Module: crate::section
// Provides: {"PanicMessage"}
// Dependencies: {}
# [doc = " Trait for printing a panic error message for the given PanicInfo"] pub trait PanicMessage : Send + Sync + 'static { # [doc = " Display trait equivalent for implementing the display logic"] fn display (& self , pi : & std :: panic :: PanicInfo < '_ > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; }
};
}

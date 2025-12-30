// Generated macro for impl_59 (impl)
macro_rules! Depcrate_smart_displayimpl_59 {
() => {
// Module: crate::smart_display
// Provides: {"impl_59"}
// Dependencies: {}
# [doc = " Permit using `Metadata` as a smart pointer to the user-provided metadata."] impl < T > Deref for Metadata < '_ , T > where T : SmartDisplay + ? Sized , { type Target = T :: Metadata ; fn deref (& self) -> & T :: Metadata { & self . metadata } }
};
}

// Generated macro for impl_173 (impl)
macro_rules! Depcrate_connectionimpl_173 {
() => {
// Module: crate::connection
// Provides: {"impl_173"}
// Dependencies: {}
impl < DB : Backend + 'static > dyn BoxableConnection < DB > { # [doc = " Downcast the current connection to a specific connection"] # [doc = " type."] # [doc = ""] # [doc = " This will return `None` if the underlying"] # [doc = " connection does not match the corresponding"] # [doc = " type, otherwise a reference to the underlying connection is returned"] pub fn downcast_ref < T > (& self) -> Option < & T > where T : Connection < Backend = DB > + 'static , { self . as_any () . downcast_ref :: < T > () } # [doc = " Downcast the current connection to a specific mutable connection"] # [doc = " type."] # [doc = ""] # [doc = " This will return `None` if the underlying"] # [doc = " connection does not match the corresponding"] # [doc = " type, otherwise a mutable reference to the underlying connection is returned"] pub fn downcast_mut < T > (& mut self) -> Option < & mut T > where T : Connection < Backend = DB > + 'static , { self . as_any_mut () . downcast_mut :: < T > () } # [doc = " Check if the current connection is"] # [doc = " a specific connection type"] pub fn is < T > (& self) -> bool where T : Connection < Backend = DB > + 'static , { self . as_any () . is :: < T > () } }
};
}

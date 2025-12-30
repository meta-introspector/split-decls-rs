// Generated macro for impl_38 (impl)
macro_rules! Depcrate_errorimpl_38 {
() => {
// Module: crate::error
// Provides: {"impl_38"}
// Dependencies: {}
impl ErrorExtensionValues { # [doc = " Set an extension value."] pub fn set (& mut self , name : impl AsRef < str > , value : impl Into < Value >) { self . 0 . insert (name . as_ref () . to_string () , value . into ()) ; } # [doc = " Unset an extension value."] pub fn unset (& mut self , name : impl AsRef < str >) { self . 0 . remove (name . as_ref ()) ; } # [doc = " Get an extension value."] pub fn get (& self , name : impl AsRef < str >) -> Option < & Value > { self . 0 . get (name . as_ref ()) } }
};
}

// Generated macro for impl_23 (impl)
macro_rules! Depcrate_fullnameimpl_23 {
() => {
// Module: crate::fullname
// Provides: {"impl_23"}
// Dependencies: {}
impl FullNameRef { # [doc = " Return the file name portion of a full name, for instance `main` if the"] # [doc = " full name was `refs/heads/main`."] pub fn file_name (& self) -> & BStr { self . 0 . rsplitn (2 , | b | * b == b'/') . next () . expect ("valid ref") . as_bstr () } }
};
}

// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < P : AsRef < Path > + ? Sized > PartialEq < P > for AbsPath { fn eq (& self , other : & P) -> bool { self . 0 . as_std_path () == other . as_ref () } }
};
}

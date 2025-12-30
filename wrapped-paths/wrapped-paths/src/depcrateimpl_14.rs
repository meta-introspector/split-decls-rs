// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < P : AsRef < Path > + ? Sized > PartialEq < P > for AbsPathBuf { fn eq (& self , other : & P) -> bool { self . 0 . as_std_path () == other . as_ref () } }
};
}

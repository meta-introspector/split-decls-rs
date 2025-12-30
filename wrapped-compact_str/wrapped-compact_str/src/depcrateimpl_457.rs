// Generated macro for impl_457 (impl)
macro_rules! Depcrateimpl_457 {
() => {
// Module: crate
// Provides: {"impl_457"}
// Dependencies: {}
impl < T : AsRef < str > + ? Sized > PartialEq < T > for CompactString { fn eq (& self , other : & T) -> bool { self . as_str () == other . as_ref () } }
};
}

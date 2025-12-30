// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : AsRef < str > > PartialEq < T > for ByteString { fn eq (& self , other : & T) -> bool { & self [..] == other . as_ref () } }
};
}

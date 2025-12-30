// Generated macro for impl_237 (impl)
macro_rules! Depcrateimpl_237 {
() => {
// Module: crate
// Provides: {"impl_237"}
// Dependencies: {}
impl DirEntry < '_ > { fn path < 'a > (& self , state : & 'a State) -> & 'a BStr { let range = self . entry . path . start .. self . dir_end ; state . path_backing [range] . as_bstr () } }
};
}

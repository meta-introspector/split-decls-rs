// Generated macro for impl_64 (impl)
macro_rules! Depcrate_header_mapimpl_64 {
() => {
// Module: crate::header::map
// Provides: {"impl_64"}
// Dependencies: {}
impl < T : PartialEq > PartialEq for HeaderMap < T > { fn eq (& self , other : & HeaderMap < T >) -> bool { if self . len () != other . len () { return false ; } self . keys () . all (| key | self . get_all (key) == other . get_all (key)) } }
};
}

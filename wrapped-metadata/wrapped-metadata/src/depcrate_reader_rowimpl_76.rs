// Generated macro for impl_76 (impl)
macro_rules! Depcrate_reader_rowimpl_76 {
() => {
// Module: crate::reader::row
// Provides: {"impl_76"}
// Dependencies: {}
impl Ord for Row < '_ > { fn cmp (& self , other : & Self) -> Ordering { (self . file , self . pos) . cmp (& (other . file , other . pos)) } }
};
}

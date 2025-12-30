// Generated macro for test (module)
macro_rules! Depcrate_datatest {
() => {
// Module: crate::data
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: CFData ; use std :: sync :: Arc ; # [test] fn test_data_provider () { let l = vec ! [5] ; CFData :: from_arc (Arc :: new (l)) ; let l = vec ! [5] ; CFData :: from_arc (Arc :: new (l . into_boxed_slice ())) ; use std :: sync :: atomic :: { AtomicBool , Ordering :: SeqCst } ; struct VecWrapper { inner : Vec < u8 > , dropped : Arc < AtomicBool > , } impl Drop for VecWrapper { fn drop (& mut self) { self . dropped . store (true , SeqCst) } } impl std :: convert :: AsRef < [u8] > for VecWrapper { fn as_ref (& self) -> & [u8] { & self . inner } } let dropped = Arc :: new (AtomicBool :: default ()) ; let l = Arc :: new (VecWrapper { inner : vec ! [5] , dropped : dropped . clone () , }) ; let m = l . clone () ; let dp = CFData :: from_arc (l) ; drop (m) ; assert ! (! dropped . load (SeqCst)) ; drop (dp) ; assert ! (dropped . load (SeqCst)) } }
};
}

// Generated macro for impl_97 (impl)
macro_rules! Depcrate_dateimpl_97 {
() => {
// Module: crate::date
// Provides: {"impl_97"}
// Dependencies: {}
impl Ord for CFDate { # [inline] # [doc (alias = "CFDateCompare")] fn cmp (& self , other : & Self) -> Ordering { let context = ptr :: null_mut () ; unsafe { self . compare (Some (other) , context) } . into () } }
};
}

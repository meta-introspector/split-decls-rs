// Generated macro for impl_1728 (impl)
macro_rules! Depcrate_numberimpl_1728 {
() => {
// Module: crate::number
// Provides: {"impl_1728"}
// Dependencies: {}
impl Ord for CFNumber { # [inline] # [doc (alias = "CFNumberCompare")] fn cmp (& self , other : & Self) -> Ordering { let context = ptr :: null_mut () ; unsafe { self . compare (Some (other) , context) } . into () } }
};
}

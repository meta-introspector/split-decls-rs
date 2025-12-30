// Generated macro for impl_413 (impl)
macro_rules! Depcrate_traitsimpl_413 {
() => {
// Module: crate::traits
// Provides: {"impl_413"}
// Dependencies: {}
impl < 'a , 'b > Compare < & 'b str > for & 'a str { # [inline (always)] fn compare (& self , t : & 'b str) -> CompareResult { self . as_bytes () . compare (t . as_bytes ()) } # [inline (always)] fn compare_no_case (& self , t : & 'b str) -> CompareResult { let pos = self . chars () . zip (t . chars ()) . position (| (a , b) | a . to_lowercase () . ne (b . to_lowercase ())) ; match pos { Some (_) => CompareResult :: Error , None => { if self . len () >= t . len () { CompareResult :: Ok } else { CompareResult :: Incomplete } } } } }
};
}

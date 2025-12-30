// Generated macro for impl_411 (impl)
macro_rules! Depcrate_traitsimpl_411 {
() => {
// Module: crate::traits
// Provides: {"impl_411"}
// Dependencies: {}
impl < 'a , 'b > Compare < & 'b [u8] > for & 'a [u8] { # [inline (always)] fn compare (& self , t : & 'b [u8]) -> CompareResult { let pos = self . iter () . zip (t . iter ()) . position (| (a , b) | a != b) ; match pos { Some (_) => CompareResult :: Error , None => { if self . len () >= t . len () { CompareResult :: Ok } else { CompareResult :: Incomplete } } } } # [inline (always)] fn compare_no_case (& self , t : & 'b [u8]) -> CompareResult { if self . iter () . zip (t) . any (| (a , b) | lowercase_byte (* a) != lowercase_byte (* b)) { CompareResult :: Error } else if self . len () < t . len () { CompareResult :: Incomplete } else { CompareResult :: Ok } } }
};
}

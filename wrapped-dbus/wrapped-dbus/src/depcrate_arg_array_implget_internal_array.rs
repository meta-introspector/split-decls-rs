// Generated macro for get_internal_array (function)
macro_rules! Depcrate_arg_array_implget_internal_array {
() => {
// Module: crate::arg::array_impl
// Provides: {"get_internal_array"}
// Dependencies: {}
fn get_internal_array (i : & mut Iter) -> Box < dyn RefArg > { let mut si = i . recurse (ArgType :: Array) . unwrap () ; let inner_sig = si . signature () ; let data = si . collect :: < Vec < _ > > () ; Box :: new (InternalArray { data , inner_sig }) }
};
}

// Generated macro for get_fixed_array_refarg (function)
macro_rules! Depcrate_arg_array_implget_fixed_array_refarg {
() => {
// Module: crate::arg::array_impl
// Provides: {"get_fixed_array_refarg"}
// Dependencies: {}
fn get_fixed_array_refarg < T : FixedArray + Clone + RefArg > (i : & mut Iter) -> Box < dyn RefArg > { let s = < & [T] > :: get (i) . unwrap () ; Box :: new (s . to_vec ()) }
};
}

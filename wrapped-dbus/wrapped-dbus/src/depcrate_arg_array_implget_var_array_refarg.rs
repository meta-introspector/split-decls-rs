// Generated macro for get_var_array_refarg (function)
macro_rules! Depcrate_arg_array_implget_var_array_refarg {
() => {
// Module: crate::arg::array_impl
// Provides: {"get_var_array_refarg"}
// Dependencies: {}
fn get_var_array_refarg < 'a , T : 'static + RefArg + Arg , F : FnMut (& mut Iter < 'a >) -> Option < T > > (i : & mut Iter < 'a > , mut f : F) -> Box < dyn RefArg > { let mut v : Vec < T > = vec ! () ; let mut si = i . recurse (ArgType :: Array) . unwrap () ; while let Some (q) = f (& mut si) { v . push (q) ; si . next () ; } Box :: new (v) }
};
}

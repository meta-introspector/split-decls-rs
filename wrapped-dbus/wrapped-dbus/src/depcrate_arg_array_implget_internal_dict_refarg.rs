// Generated macro for get_internal_dict_refarg (function)
macro_rules! Depcrate_arg_array_implget_internal_dict_refarg {
() => {
// Module: crate::arg::array_impl
// Provides: {"get_internal_dict_refarg"}
// Dependencies: {}
fn get_internal_dict_refarg < 'a , K , F : FnMut (& mut Iter < 'a >) -> Option < K > > (i : & mut Iter < 'a > , mut f : F ,) -> Box < dyn RefArg > where K : DictKey + 'static + RefArg + Clone , { let mut data = vec ! [] ; let outer_sig = i . signature () ; let mut si = i . recurse (ArgType :: Array) . unwrap () ; while let Some (mut d) = si . recurse (ArgType :: DictEntry) { let k = f (& mut d) . unwrap () ; d . next () ; data . push ((k , d . get_refarg () . unwrap ())) ; si . next () ; } Box :: new (InternalDict { data , outer_sig }) }
};
}

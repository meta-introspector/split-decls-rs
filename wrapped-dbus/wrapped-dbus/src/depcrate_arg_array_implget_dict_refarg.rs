// Generated macro for get_dict_refarg (function)
macro_rules! Depcrate_arg_array_implget_dict_refarg {
() => {
// Module: crate::arg::array_impl
// Provides: {"get_dict_refarg"}
// Dependencies: {}
fn get_dict_refarg < 'a , K , V , KF , VF > (i : & mut Iter < 'a > , mut kf : KF , mut vf : VF) -> Box < dyn RefArg > where K : DictKey + 'static + RefArg + Clone + Eq + Hash , V : RefArg + Arg + 'static , KF : FnMut (& mut Iter < 'a >) -> Option < K > , VF : FnMut (& mut Iter < 'a >) -> Option < V > , { let mut data : HashMap < K , V > = HashMap :: new () ; let mut si = i . recurse (ArgType :: Array) . unwrap () ; while let Some (mut d) = si . recurse (ArgType :: DictEntry) { let k = kf (& mut d) . unwrap () ; d . next () ; let v = vf (& mut d) . unwrap () ; data . insert (k , v) ; si . next () ; } Box :: new (data) }
};
}

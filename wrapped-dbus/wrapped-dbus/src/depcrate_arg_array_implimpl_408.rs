// Generated macro for impl_408 (impl)
macro_rules! Depcrate_arg_array_implimpl_408 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_408"}
// Dependencies: {}
impl < 'a , K : DictKey + Get < 'a > , V : Arg + Get < 'a > > Iterator for Dict < 'a , K , V , Iter < 'a > > { type Item = (K , V) ; fn next (& mut self) -> Option < (K , V) > { let i = self . 0 . recurse (ArgType :: DictEntry) . and_then (| mut si | { let k = si . get () ; if k . is_none () { return None } ; assert ! (si . next ()) ; let v = si . get () ; if v . is_none () { return None } ; Some ((k . unwrap () , v . unwrap ())) }) ; self . 0 . next () ; i } }
};
}

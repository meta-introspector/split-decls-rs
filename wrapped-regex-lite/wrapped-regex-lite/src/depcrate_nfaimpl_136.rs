// Generated macro for impl_136 (impl)
macro_rules! Depcrate_nfaimpl_136 {
() => {
// Module: crate::nfa
// Provides: {"impl_136"}
// Dependencies: {}
impl State { # [doc = " Returns the heap memory usage of this NFA state in bytes."] fn memory_usage (& self) -> usize { match * self { State :: Char { .. } | State :: Goto { .. } | State :: Capture { .. } | State :: Fail { .. } | State :: Match => 0 , State :: Splits { ref targets , .. } => { targets . len () * size_of :: < StateID > () } State :: Ranges { ref ranges , .. } => { ranges . len () * size_of :: < (char , char) > () } } } # [doc = " Returns an iterator over the given split targets. The order of the"] # [doc = " iterator yields elements in reverse when `reverse` is true."] pub (crate) fn iter_splits < 'a > (splits : & 'a [StateID] , reverse : bool ,) -> impl Iterator < Item = StateID > + 'a { let mut it = splits . iter () ; core :: iter :: from_fn (move | | { if reverse { it . next_back () } else { it . next () } . copied () }) } }
};
}

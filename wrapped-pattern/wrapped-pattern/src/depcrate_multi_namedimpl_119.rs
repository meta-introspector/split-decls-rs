// Generated macro for impl_119 (impl)
macro_rules! Depcrate_multi_namedimpl_119 {
() => {
// Module: crate::multi_named
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'a > Iterator for MultiNamedPlaceholderPatternIterator < 'a > { type Item = PatternItem < 'a , MultiNamedPlaceholderKey < 'a > > ; fn next (& mut self) -> Option < Self :: Item > { match self . try_next () { Ok (next) => next , Err (MultiNamedPlaceholderError :: InvalidStore) => { debug_assert ! (false , "invalid store with {} bytes remaining" , self . store . len ()) ; None } Err (MultiNamedPlaceholderError :: Unreachable) => { debug_assert ! (false , "unreachable") ; None } } } }
};
}

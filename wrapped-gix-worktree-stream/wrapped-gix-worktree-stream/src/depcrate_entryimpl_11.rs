// Generated macro for impl_11 (impl)
macro_rules! Depcrate_entryimpl_11 {
() => {
// Module: crate::entry
// Provides: {"impl_11"}
// Dependencies: {}
impl Source { pub (crate) fn len (& self) -> Option < usize > { match self { Source :: Null => Some (0) , Source :: Path (_) => None , Source :: Memory (buf) => Some (buf . len ()) , } } }
};
}

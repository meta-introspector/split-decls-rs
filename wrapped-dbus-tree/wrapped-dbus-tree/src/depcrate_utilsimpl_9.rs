// Generated macro for impl_9 (impl)
macro_rules! Depcrate_utilsimpl_9 {
() => {
// Module: crate::utils
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , V : 'a > Iterator for Iter < 'a , V > { type Item = & 'a Arc < V > ; fn next (& mut self) -> Option < Self :: Item > { match self . 0 { IterE :: Path (ref mut x) => x . next () , IterE :: Iface (ref mut x) => x . next () , IterE :: Member (ref mut x) => x . next () , IterE :: String (ref mut x) => x . next () , } } }
};
}

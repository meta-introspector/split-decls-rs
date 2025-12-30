// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl PartialOrd for Signature { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { (self . r () , self . s ()) . partial_cmp (& (other . r () , other . s ())) } }
};
}

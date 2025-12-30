// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl PartialOrd for Signature { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { (self . r () , self . s ()) . partial_cmp (& (other . r () , other . s ())) } }
};
}

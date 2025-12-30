// Generated macro for impl_3657 (impl)
macro_rules! Depcrate_loops_manual_memcpyimpl_3657 {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"impl_3657"}
// Dependencies: {}
impl std :: ops :: Sub for & MinifyingSugg < 'static > { type Output = MinifyingSugg < 'static > ; fn sub (self , rhs : & MinifyingSugg < 'static >) -> MinifyingSugg < 'static > { match (self . to_string () . as_str () , rhs . to_string () . as_str ()) { (_ , "0") => self . clone () , ("0" , _) => (- rhs . 0 . clone ()) . into () , (x , y) if x == y => sugg :: ZERO . into () , (_ , _) => (& self . 0 - & rhs . 0) . into () , } } }
};
}

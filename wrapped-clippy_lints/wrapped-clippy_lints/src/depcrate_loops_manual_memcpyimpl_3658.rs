// Generated macro for impl_3658 (impl)
macro_rules! Depcrate_loops_manual_memcpyimpl_3658 {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"impl_3658"}
// Dependencies: {}
impl std :: ops :: Add < & MinifyingSugg < 'static > > for MinifyingSugg < 'static > { type Output = MinifyingSugg < 'static > ; fn add (self , rhs : & MinifyingSugg < 'static >) -> MinifyingSugg < 'static > { match (self . to_string () . as_str () , rhs . to_string () . as_str ()) { ("0" , _) => rhs . clone () , (_ , "0") => self , (_ , _) => (self . 0 + & rhs . 0) . into () , } } }
};
}

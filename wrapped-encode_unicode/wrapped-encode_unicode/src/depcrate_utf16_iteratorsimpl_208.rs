// Generated macro for impl_208 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_208 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_208"}
// Dependencies: {}
impl fmt :: Debug for Utf16Iterator { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { let mut clone = self . clone () ; match (clone . next () , clone . next ()) { (Some (one) , None) => write ! (fmtr , "[{}]" , one) , (Some (a) , Some (b)) => write ! (fmtr , "[{}, {}]" , a , b) , (None , _) => write ! (fmtr , "[]") , } } }
};
}

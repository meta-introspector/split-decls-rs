// Generated macro for impl_110 (impl)
macro_rules! Depcrate_dfaimpl_110 {
() => {
// Module: crate::dfa
// Provides: {"impl_110"}
// Dependencies: {}
impl fmt :: Debug for StateFlags { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("StateFlags") . field ("is_match" , & self . is_match ()) . field ("is_word" , & self . is_word ()) . field ("has_empty" , & self . has_empty ()) . finish () } }
};
}

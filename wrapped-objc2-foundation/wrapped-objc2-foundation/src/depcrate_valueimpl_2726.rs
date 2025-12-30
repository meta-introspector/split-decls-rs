// Generated macro for impl_2726 (impl)
macro_rules! Depcrate_valueimpl_2726 {
() => {
// Module: crate::value
// Provides: {"impl_2726"}
// Dependencies: {}
impl fmt :: Debug for NSValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let enc = self . encoding () . unwrap_or ("(NULL)") ; let bytes = & * * self ; f . debug_struct ("NSValue") . field ("encoding" , & enc) . field ("bytes" , bytes) . finish () } }
};
}

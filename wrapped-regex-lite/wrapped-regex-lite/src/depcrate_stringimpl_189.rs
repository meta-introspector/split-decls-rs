// Generated macro for impl_189 (impl)
macro_rules! Depcrate_stringimpl_189 {
() => {
// Module: crate::string
// Provides: {"impl_189"}
// Dependencies: {}
impl core :: fmt :: Debug for Regex { # [doc = " Shows the original regular expression."] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Regex") . field (& self . as_str ()) . finish () } }
};
}

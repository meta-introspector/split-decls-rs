// Generated macro for impl_101 (impl)
macro_rules! Depcrate_regex_stringimpl_101 {
() => {
// Module: crate::regex::string
// Provides: {"impl_101"}
// Dependencies: {}
impl core :: fmt :: Debug for Regex { # [doc = " Shows the original regular expression."] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Regex") . field (& self . as_str ()) . finish () } }
};
}

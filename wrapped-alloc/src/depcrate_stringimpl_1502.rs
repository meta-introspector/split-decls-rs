// Generated macro for impl_1502 (impl)
macro_rules! Depcrate_stringimpl_1502 {
() => {
// Module: crate::string
// Provides: {"impl_1502"}
// Dependencies: {}
# [unstable (feature = "string_into_chars" , issue = "133125")] impl fmt :: Debug for IntoChars { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoChars") . field (& self . as_str ()) . finish () } }
};
}

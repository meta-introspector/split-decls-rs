// Generated macro for impl_110 (impl)
macro_rules! Depcrate_regex_stringimpl_110 {
() => {
// Module: crate::regex::string
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'h > core :: fmt :: Debug for Match < 'h > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Match") . field ("start" , & self . start) . field ("end" , & self . end) . field ("string" , & self . as_str ()) . finish () } }
};
}

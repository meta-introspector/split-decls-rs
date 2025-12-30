// Generated macro for impl_198 (impl)
macro_rules! Depcrate_stringimpl_198 {
() => {
// Module: crate::string
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'h > core :: fmt :: Debug for Match < 'h > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_struct ("Match") . field ("start" , & self . start) . field ("end" , & self . end) . field ("string" , & self . as_str ()) . finish () } }
};
}

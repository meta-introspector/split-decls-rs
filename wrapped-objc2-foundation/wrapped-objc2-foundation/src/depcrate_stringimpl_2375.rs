// Generated macro for impl_2375 (impl)
macro_rules! Depcrate_stringimpl_2375 {
() => {
// Module: crate::string
// Provides: {"impl_2375"}
// Dependencies: {}
impl fmt :: Debug for NSString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { autoreleasepool_leaking (| pool | fmt :: Debug :: fmt (unsafe { nsstring_to_str (self , pool) } , f)) } }
};
}

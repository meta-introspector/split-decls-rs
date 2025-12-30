// Generated macro for impl_2377 (impl)
macro_rules! Depcrate_stringimpl_2377 {
() => {
// Module: crate::string
// Provides: {"impl_2377"}
// Dependencies: {}
impl fmt :: Write for & NSMutableString { fn write_str (& mut self , s : & str) -> fmt :: Result { let nsstring = NSString :: from_str (s) ; self . appendString (& nsstring) ; Ok (()) } }
};
}

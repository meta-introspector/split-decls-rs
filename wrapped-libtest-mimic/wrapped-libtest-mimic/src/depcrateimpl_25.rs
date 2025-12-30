// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Debug for Trial { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct OpaqueRunner ; impl fmt :: Debug for OpaqueRunner { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<runner>") } } f . debug_struct ("Test") . field ("runner" , & OpaqueRunner) . field ("name" , & self . info . name) . field ("kind" , & self . info . kind) . field ("is_ignored" , & self . info . is_ignored) . field ("is_bench" , & self . info . is_bench) . finish () } }
};
}

// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Debug for Errno { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { sys :: with_description (* self , | desc | { fmt . debug_struct ("Errno") . field ("code" , & self . 0) . field ("description" , & desc . ok ()) . finish () }) } }
};
}

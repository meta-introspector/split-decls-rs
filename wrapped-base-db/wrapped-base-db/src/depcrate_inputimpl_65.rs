// Generated macro for impl_65 (impl)
macro_rules! Depcrate_inputimpl_65 {
() => {
// Module: crate::input
// Provides: {"impl_65"}
// Dependencies: {}
impl fmt :: Debug for Env { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct EnvDebug < 's > (Vec < (& 's String , & 's String) >) ; impl fmt :: Debug for EnvDebug < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . 0 . iter () . copied ()) . finish () } } f . debug_struct ("Env") . field ("entries" , & { let mut entries : Vec < _ > = self . entries . iter () . collect () ; entries . sort () ; EnvDebug (entries) }) . finish () } }
};
}

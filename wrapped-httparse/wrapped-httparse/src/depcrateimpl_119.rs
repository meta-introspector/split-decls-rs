// Generated macro for impl_119 (impl)
macro_rules! Depcrateimpl_119 {
() => {
// Module: crate
// Provides: {"impl_119"}
// Dependencies: {}
impl fmt :: Debug for Header < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("Header") ; f . field ("name" , & self . name) ; if let Ok (value) = str :: from_utf8 (self . value) { f . field ("value" , & value) ; } else { f . field ("value" , & self . value) ; } f . finish () } }
};
}

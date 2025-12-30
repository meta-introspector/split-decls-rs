// Generated macro for impl_2297 (impl)
macro_rules! Depcrate_numberimpl_2297 {
() => {
// Module: crate::number
// Provides: {"impl_2297"}
// Dependencies: {}
impl fmt :: Display for NSNumber { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let string : Retained < NSObject > = unsafe { msg_send ! [self , stringValue] } ; unsafe { util :: display_string (& string , f) } } }
};
}

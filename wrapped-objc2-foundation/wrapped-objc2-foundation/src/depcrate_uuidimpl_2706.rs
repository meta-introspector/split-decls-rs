// Generated macro for impl_2706 (impl)
macro_rules! Depcrate_uuidimpl_2706 {
() => {
// Module: crate::uuid
// Provides: {"impl_2706"}
// Dependencies: {}
impl fmt :: Display for NSUUID { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let string : Retained < NSObject > = unsafe { msg_send ! [self , UUIDString] } ; unsafe { util :: display_string (& string , f) } } }
};
}

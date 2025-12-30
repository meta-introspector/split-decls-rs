// Generated macro for impl_2707 (impl)
macro_rules! Depcrate_uuidimpl_2707 {
() => {
// Module: crate::uuid
// Provides: {"impl_2707"}
// Dependencies: {}
impl fmt :: Debug for NSUUID { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let string : Retained < NSObject > = unsafe { msg_send ! [self , UUIDString] } ; unsafe { util :: display_string (& string , f) } } }
};
}

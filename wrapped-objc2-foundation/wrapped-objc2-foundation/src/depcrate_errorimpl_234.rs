// Generated macro for impl_234 (impl)
macro_rules! Depcrate_errorimpl_234 {
() => {
// Module: crate::error
// Provides: {"impl_234"}
// Dependencies: {}
impl fmt :: Display for NSError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let desc : Retained < NSObject > = if cfg ! (feature = "gnustep-1-7") { let desc : Option < Retained < NSObject > > = unsafe { msg_send ! [self , localizedDescription] } ; if let Some (desc) = desc { desc } else { let domain : Retained < NSObject > = unsafe { msg_send ! [self , domain] } ; unsafe { util :: display_string (& domain , f) ? } ; write ! (f , " {}" , self . code ()) ? ; return Ok (()) ; } } else { unsafe { msg_send ! [self , localizedDescription] } } ; unsafe { util :: display_string (& desc , f) } } }
};
}

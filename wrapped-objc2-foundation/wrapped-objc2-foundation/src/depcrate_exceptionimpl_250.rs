// Generated macro for impl_250 (impl)
macro_rules! Depcrate_exceptionimpl_250 {
() => {
// Module: crate::exception
// Provides: {"impl_250"}
// Dependencies: {}
impl fmt :: Debug for NSException { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let obj : & AnyObject = self . as_ref () ; write ! (f , "{obj:?}") ? ; write ! (f , " '") ? ; let name : Retained < NSObject > = unsafe { msg_send ! [self , name] } ; unsafe { util :: display_string (& name , f) ? } ; write ! (f , "'") ? ; write ! (f , " reason: ") ? ; let reason : Option < Retained < NSObject > > = unsafe { msg_send ! [self , reason] } ; if let Some (reason) = reason { unsafe { util :: display_string (& reason , f) ? } ; } else { write ! (f , "(NULL)") ? ; } Ok (()) } }
};
}

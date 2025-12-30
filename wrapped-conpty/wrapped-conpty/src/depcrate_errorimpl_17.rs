// Generated macro for impl_17 (impl)
macro_rules! Depcrate_errorimpl_17 {
() => {
// Module: crate::error
// Provides: {"impl_17"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Win (err) => writeln ! (f , "Windows error: {}" , err) , Self :: Timeout (limit) => writeln ! (f , "A timeout {:?} was reached" , limit) , Self :: WaitFailed (event_id) => writeln ! (f , "Waiting failed. WAIT_EVENT: {:?}" , event_id) , Self :: InputClosed => writeln ! (f , "The input is already closed") , } } }
};
}

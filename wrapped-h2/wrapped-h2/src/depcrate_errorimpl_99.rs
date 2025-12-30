// Generated macro for impl_99 (impl)
macro_rules! Depcrate_errorimpl_99 {
() => {
// Module: crate::error
// Provides: {"impl_99"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let debug_data = match self . kind { Kind :: Reset (_ , reason , Initiator :: User) => { return write ! (fmt , "stream error sent by user: {}" , reason) } Kind :: Reset (_ , reason , Initiator :: Library) => { return write ! (fmt , "stream error detected: {}" , reason) } Kind :: Reset (_ , reason , Initiator :: Remote) => { return write ! (fmt , "stream error received: {}" , reason) } Kind :: GoAway (ref debug_data , reason , Initiator :: User) => { write ! (fmt , "connection error sent by user: {}" , reason) ? ; debug_data } Kind :: GoAway (ref debug_data , reason , Initiator :: Library) => { write ! (fmt , "connection error detected: {}" , reason) ? ; debug_data } Kind :: GoAway (ref debug_data , reason , Initiator :: Remote) => { write ! (fmt , "connection error received: {}" , reason) ? ; debug_data } Kind :: Reason (reason) => return write ! (fmt , "protocol error: {}" , reason) , Kind :: User (ref e) => return write ! (fmt , "user error: {}" , e) , Kind :: Io (ref e) => return e . fmt (fmt) , } ; if ! debug_data . is_empty () { write ! (fmt , " ({:?})" , debug_data) ? ; } Ok (()) } }
};
}

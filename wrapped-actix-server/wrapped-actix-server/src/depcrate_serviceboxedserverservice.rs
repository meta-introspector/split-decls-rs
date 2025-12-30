// Generated macro for BoxedServerService (type)
macro_rules! Depcrate_serviceBoxedServerService {
() => {
// Module: crate::service
// Provides: {"BoxedServerService"}
// Dependencies: {}
pub (crate) type BoxedServerService = Box < dyn Service < (WorkerCounterGuard , MioStream) , Response = () , Error = () , Future = Ready < Result < () , () > > , > , > ;
};
}

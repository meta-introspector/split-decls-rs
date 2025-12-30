// Generated macro for BoxServiceFactory (struct)
macro_rules! DepcrateBoxServiceFactory {
() => {
// Module: crate
// Provides: {"BoxServiceFactory"}
// Dependencies: {}
struct BoxServiceFactory < Req > (Box < dyn ServiceFactory < Req , Service = Box < dyn Service < Req , Error = () , Future = BoxFuture < Result < () , () > > > > , Future = BoxFuture < Result < Box < dyn Service < Req , Error = () , Future = BoxFuture < Result < () , () > > > > , () > , > , > , > ,) ;
};
}

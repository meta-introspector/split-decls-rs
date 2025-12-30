// Generated macro for ServiceFactory (trait)
macro_rules! DepcrateServiceFactory {
() => {
// Module: crate
// Provides: {"ServiceFactory"}
// Dependencies: {}
trait ServiceFactory < Req > { type Service : Service < Req > ; type Future ; fn new_service (& self , _ : ()) -> Self :: Future ; }
};
}

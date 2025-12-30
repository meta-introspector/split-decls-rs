// Generated macro for Service (trait)
macro_rules! DepcrateService {
() => {
// Module: crate
// Provides: {"Service"}
// Dependencies: {}
trait Service < Req > { type Error ; type Future ; fn poll (& self , ctx : & mut std :: task :: Context) -> std :: task :: Poll < Result < () , Self :: Error > > ; fn call (& self , req : Req) -> Self :: Future ; }
};
}

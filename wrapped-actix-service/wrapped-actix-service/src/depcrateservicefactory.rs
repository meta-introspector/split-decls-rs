// Generated macro for ServiceFactory (trait)
macro_rules! DepcrateServiceFactory {
() => {
// Module: crate
// Provides: {"ServiceFactory"}
// Dependencies: {}
# [doc = " Factory for creating `Service`s."] # [doc = ""] # [doc = " This is useful for cases where new `Service`s must be produced. One case is a TCP"] # [doc = " server listener: a listener accepts new connections, constructs a new `Service` for each using"] # [doc = " the `ServiceFactory` trait, and uses the new `Service` to process inbound requests on that new"] # [doc = " connection."] # [doc = ""] # [doc = " `Config` is a service factory configuration type."] # [doc = ""] # [doc = " Simple factories may be able to use [`fn_factory`] or [`fn_factory_with_config`] to"] # [doc = " reduce boilerplate."] pub trait ServiceFactory < Req > { # [doc = " Responses given by the created services."] type Response ; # [doc = " Errors produced by the created services."] type Error ; # [doc = " Service factory configuration."] type Config ; # [doc = " The kind of `Service` created by this factory."] type Service : Service < Req , Response = Self :: Response , Error = Self :: Error > ; # [doc = " Errors potentially raised while building a service."] type InitError ; # [doc = " The future of the `Service` instance.g"] type Future : Future < Output = Result < Self :: Service , Self :: InitError > > ; # [doc = " Create and return a new service asynchronously."] fn new_service (& self , cfg : Self :: Config) -> Self :: Future ; }
};
}

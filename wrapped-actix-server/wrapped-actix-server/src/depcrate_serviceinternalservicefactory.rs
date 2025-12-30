// Generated macro for InternalServiceFactory (trait)
macro_rules! Depcrate_serviceInternalServiceFactory {
() => {
// Module: crate::service
// Provides: {"InternalServiceFactory"}
// Dependencies: {}
pub (crate) trait InternalServiceFactory : Send { fn name (& self , token : usize) -> & str ; fn clone_factory (& self) -> Box < dyn InternalServiceFactory > ; fn create (& self) -> LocalBoxFuture < 'static , Result < (usize , BoxedServerService) , () > > ; }
};
}

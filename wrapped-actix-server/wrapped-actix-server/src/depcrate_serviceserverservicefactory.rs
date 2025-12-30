// Generated macro for ServerServiceFactory (trait)
macro_rules! Depcrate_serviceServerServiceFactory {
() => {
// Module: crate::service
// Provides: {"ServerServiceFactory"}
// Dependencies: {}
# [doc (hidden)] pub trait ServerServiceFactory < Stream : FromStream > : Send + Clone + 'static { type Factory : BaseServiceFactory < Stream , Config = () > ; fn create (& self) -> Self :: Factory ; }
};
}

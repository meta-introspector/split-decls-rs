// Generated macro for new_schema (function)
macro_rules! Depcratenew_schema {
() => {
// Module: crate
// Provides: {"new_schema"}
// Dependencies: {}
pub fn new_schema () -> RootNode < Query , EmptyMutation < Context > , EmptySubscription < Context > > { RootNode :: new (Query , EmptyMutation :: new () , EmptySubscription :: new ()) }
};
}

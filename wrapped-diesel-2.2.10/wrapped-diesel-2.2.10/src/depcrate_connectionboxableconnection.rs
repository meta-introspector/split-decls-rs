// Generated macro for BoxableConnection (trait)
macro_rules! Depcrate_connectionBoxableConnection {
() => {
// Module: crate::connection
// Provides: {"BoxableConnection"}
// Dependencies: {}
# [doc = " A variant of the [`Connection`](trait.Connection.html) trait that is"] # [doc = " usable with dynamic dispatch"] # [doc = ""] # [doc = " If you are looking for a way to use pass database connections"] # [doc = " for different database backends around in your application"] # [doc = " this trait won't help you much. Normally you should only"] # [doc = " need to use this trait if you are interacting with a connection"] # [doc = " passed to a [`Migration`](../migration/trait.Migration.html)"] pub trait BoxableConnection < DB : Backend > : SimpleConnection + std :: any :: Any { # [doc = " Maps the current connection to `std::any::Any`"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] fn as_any (& self) -> & dyn std :: any :: Any ; # [doc (hidden)] fn as_any_mut (& mut self) -> & mut dyn std :: any :: Any ; }
};
}

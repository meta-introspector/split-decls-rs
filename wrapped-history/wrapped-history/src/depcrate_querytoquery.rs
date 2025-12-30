// Generated macro for ToQuery (trait)
macro_rules! Depcrate_queryToQuery {
() => {
// Module: crate::query
// Provides: {"ToQuery"}
// Dependencies: {}
# [doc = " Type that can be encoded into a query string."] pub trait ToQuery { # [doc = " Error that can be returned from the conversion."] type Error ; # [doc = " Method to encode the query into a string."] fn to_query (& self) -> Result < Cow < '_ , str > , Self :: Error > ; }
};
}

// Generated macro for FromQuery (trait)
macro_rules! Depcrate_queryFromQuery {
() => {
// Module: crate::query
// Provides: {"FromQuery"}
// Dependencies: {}
# [doc = " Type that can be decoded from a query string."] pub trait FromQuery { # [doc = " Target type after parsing."] type Target ; # [doc = " Error that can occur while parsing."] type Error ; # [doc = " Decode this query string into the target type."] fn from_query (query : & str) -> Result < Self :: Target , Self :: Error > ; }
};
}

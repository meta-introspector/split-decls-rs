// Generated macro for EdgeNameType (trait)
macro_rules! Depcrate_types_connectionEdgeNameType {
() => {
// Module: crate::types::connection
// Provides: {"EdgeNameType"}
// Dependencies: {}
# [doc = " Used to specify the edge name."] pub trait EdgeNameType : Send + Sync { # [doc = " Returns the edge type name."] fn type_name < T : OutputType > () -> String ; }
};
}

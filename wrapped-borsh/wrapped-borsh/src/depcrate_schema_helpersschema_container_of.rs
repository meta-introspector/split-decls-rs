// Generated macro for schema_container_of (function)
macro_rules! Depcrate_schema_helpersschema_container_of {
() => {
// Module: crate::schema_helpers
// Provides: {"schema_container_of"}
// Dependencies: {}
# [doc = " generate [BorshSchemaContainer] for type `T`"] # [doc = ""] # [doc = " this is an alias of [BorshSchemaContainer::for_type]"] pub fn schema_container_of < T : BorshSchema + ? Sized > () -> BorshSchemaContainer { BorshSchemaContainer :: for_type :: < T > () }
};
}

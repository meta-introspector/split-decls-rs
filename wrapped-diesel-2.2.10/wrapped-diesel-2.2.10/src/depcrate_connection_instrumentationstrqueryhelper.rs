// Generated macro for StrQueryHelper (struct)
macro_rules! Depcrate_connection_instrumentationStrQueryHelper {
() => {
// Module: crate::connection::instrumentation
// Provides: {"StrQueryHelper"}
// Dependencies: {}
# [doc = " A helper type that allows printing out str slices"] # [doc = ""] # [doc = " This type is necessary because it's not possible"] # [doc = " to cast from a reference of a unsized type like `&str`"] # [doc = " to a reference of a trait object even if that"] # [doc = " type implements all necessary traits"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub (crate) struct StrQueryHelper < 'query > { s : & 'query str , }
};
}

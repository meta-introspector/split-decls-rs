// Generated macro for WithMetadataLookup (trait)
macro_rules! Depcrate_connectionWithMetadataLookup {
() => {
// Module: crate::connection
// Provides: {"WithMetadataLookup"}
// Dependencies: {}
# [doc = " Describes a connection with an underlying [`crate::sql_types::TypeMetadata::MetadataLookup`]"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub trait WithMetadataLookup : Connection { # [doc = " Retrieves the underlying metadata lookup"] fn metadata_lookup (& mut self) -> & mut < Self :: Backend as TypeMetadata > :: MetadataLookup ; }
};
}

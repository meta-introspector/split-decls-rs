// Generated macro for IntoOwnedRow (trait)
macro_rules! Depcrate_rowIntoOwnedRow {
() => {
// Module: crate::row
// Provides: {"IntoOwnedRow"}
// Dependencies: {}
# [doc = " A row that can be turned into an owned version"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub trait IntoOwnedRow < 'a , DB : Backend > : Row < 'a , DB > { # [doc = " The owned version of the row"] type OwnedRow : Row < 'a , DB > + Send + 'static ; # [doc = " A store for cached information between rows for faster access"] type Cache : Default + 'static ; # [doc = " Turn the row into its owned version"] fn into_owned (self , cache : & mut Self :: Cache) -> Self :: OwnedRow ; }
};
}

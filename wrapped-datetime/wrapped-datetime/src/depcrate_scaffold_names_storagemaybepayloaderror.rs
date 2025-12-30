// Generated macro for MaybePayloadError (enum)
macro_rules! Depcrate_scaffold_names_storageMaybePayloadError {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"MaybePayloadError"}
// Dependencies: {}
# [doc = " An error returned by [`MaybePayload`]."] # [allow (missing_docs)] # [derive (Debug , Copy , Clone , displaydoc :: Display)] # [non_exhaustive] pub enum MaybePayloadError { # [doc = " The container's field set doesn't support the field"] FormatterTooSpecific , # [doc = " The field is already loaded with a different length"] ConflictingField (ErrorField) , }
};
}

// Generated macro for impl_1151 (impl)
macro_rules! Depcrate_scaffold_names_storageimpl_1151 {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"impl_1151"}
// Dependencies: {}
impl MaybePayloadError { pub (crate) fn into_load_error (self , error_field : ErrorField) -> PatternLoadError { match self { Self :: FormatterTooSpecific => PatternLoadError :: FormatterTooSpecific (error_field) , Self :: ConflictingField (loaded_field) => PatternLoadError :: ConflictingField { field : error_field , previous_field : loaded_field , } , } } }
};
}

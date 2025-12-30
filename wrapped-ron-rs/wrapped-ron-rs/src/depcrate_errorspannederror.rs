// Generated macro for SpannedError (struct)
macro_rules! Depcrate_errorSpannedError {
() => {
// Module: crate::error
// Provides: {"SpannedError"}
// Dependencies: {}
# [doc = " This type represents all possible errors that can occur when"] # [doc = " serializing or deserializing RON data."] # [allow (clippy :: module_name_repetitions)] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SpannedError { pub code : Error , pub span : Span , }
};
}

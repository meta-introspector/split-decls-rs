// Generated macro for LocalizationError (enum)
macro_rules! Depcrate_errorsLocalizationError {
() => {
// Module: crate::errors
// Provides: {"LocalizationError"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] pub enum LocalizationError { Bundle { error : FluentError , } , Resolver { id : String , locale : LanguageIdentifier , errors : Vec < FluentError > , } , MissingMessage { id : String , locale : Option < LanguageIdentifier > , } , MissingValue { id : String , locale : Option < LanguageIdentifier > , } , SyncRequestInAsyncMode , }
};
}

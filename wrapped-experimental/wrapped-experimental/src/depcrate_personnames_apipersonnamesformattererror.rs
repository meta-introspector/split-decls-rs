// Generated macro for PersonNamesFormatterError (enum)
macro_rules! Depcrate_personnames_apiPersonNamesFormatterError {
() => {
// Module: crate::personnames::api
// Provides: {"PersonNamesFormatterError"}
// Dependencies: {}
# [doc = ""] # [doc = " Error handling for the person name formatter."] # [derive (Clone , Eq , PartialEq , Debug , Display)] pub enum PersonNamesFormatterError { # [displaydoc ("{0}")] ParseError (String) , # [displaydoc ("Invalid person name")] InvalidPersonName , # [displaydoc ("Invalid person name")] InvalidLocale , # [displaydoc ("Invalid CLDR data")] InvalidCldrData , # [displaydoc ("{0}")] Data (DataError) , # [displaydoc ("{0}")] Pattern (icu_pattern :: Error) , }
};
}

// Generated macro for DataLocaleFamilyParseError (enum)
macro_rules! Depcrate_locale_familyDataLocaleFamilyParseError {
() => {
// Module: crate::locale_family
// Provides: {"DataLocaleFamilyParseError"}
// Dependencies: {}
# [doc = " An error while parsing a [`DataLocaleFamily`]."] # [derive (Debug , Copy , Clone , PartialEq , Display)] # [non_exhaustive] pub enum DataLocaleFamilyParseError { # [doc = " An error bubbled up from parsing a [`DataLocale`]."] # [displaydoc ("{0}")] Locale (ParseError) , # [doc = " Some other error specific to parsing the family, such as an invalid lead byte."] # [displaydoc ("Invalid locale family")] InvalidFamily , }
};
}

// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_restricted_namesErrorKind {
() => {
// Module: crate::restricted_names
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Non-public error kind for [`NameValidationError`]."] # [non_exhaustive] # [derive (Debug , thiserror :: Error)] enum ErrorKind { # [error ("{0} cannot be empty")] Empty (& 'static str) , # [error ("invalid character `{ch}` in {what}: `{name}`, {reason}")] InvalidCharacter { ch : char , what : & 'static str , name : String , reason : & 'static str , } , # [error ("profile name `{name}` is reserved\n{help}\n\
         See https://doc.rust-lang.org/cargo/reference/profiles.html \
         for more on configuring profiles.")] ProfileNameReservedKeyword { name : String , help : & 'static str } , # [error ("feature named `{0}` is not allowed to start with `dep:`")] FeatureNameStartsWithDepColon (String) , }
};
}

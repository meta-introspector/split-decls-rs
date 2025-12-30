// Generated macro for ConversionErrorKind (enum)
macro_rules! DepcrateConversionErrorKind {
() => {
// Module: crate
// Provides: {"ConversionErrorKind"}
// Dependencies: {}
# [doc = " Errors returned when trying to retrieve a literal unescaped value."] # [unstable (feature = "proc_macro_value" , issue = "136652")] # [derive (Debug , PartialEq , Eq)] pub enum ConversionErrorKind { # [doc = " The literal failed to be escaped, take a look at [`EscapeError`] for more information."] FailedToUnescape (EscapeError) , # [doc = " Trying to convert a literal with the wrong type."] InvalidLiteralKind , }
};
}

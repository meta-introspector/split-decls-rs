// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_error_kindErrorKind {
() => {
// Module: crate::error::kind
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug , Clone)] # [cfg_attr (test , derive (PartialEq))] pub (in crate :: error) enum ErrorKind { # [doc = " An arbitrary error message."] Custom (String) , DuplicateField (FieldName) , MissingField (FieldName) , UnsupportedShape { observed : DeriveInputShape , expected : Option < String > , } , UnknownField (Box < ErrorUnknownValue >) , UnexpectedFormat (MetaFormat) , UnexpectedType (String) , UnknownValue (Box < ErrorUnknownValue >) , TooFewItems (usize) , TooManyItems (usize) , # [doc = " A set of errors."] Multiple (Vec < Error >) , # [doc (hidden)] __NonExhaustive , }
};
}

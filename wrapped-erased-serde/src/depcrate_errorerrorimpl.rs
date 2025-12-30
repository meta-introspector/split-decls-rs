// Generated macro for ErrorImpl (enum)
macro_rules! Depcrate_errorErrorImpl {
() => {
// Module: crate::error
// Provides: {"ErrorImpl"}
// Dependencies: {}
enum ErrorImpl { Custom (String) , InvalidType { unexpected : Unexpected , expected : String , } , InvalidValue { unexpected : Unexpected , expected : String , } , InvalidLength { len : usize , expected : String , } , UnknownVariant { variant : String , expected : & 'static [& 'static str] , } , UnknownField { field : String , expected : & 'static [& 'static str] , } , MissingField { field : & 'static str , } , DuplicateField { field : & 'static str , } , }
};
}

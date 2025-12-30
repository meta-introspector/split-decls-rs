// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Enum representing the potential errors that TinyTemplate can encounter."] # [derive (Debug)] pub enum Error { ParseError { msg : String , line : usize , column : usize , } , RenderError { msg : String , line : usize , column : usize , } , SerdeError { err : SerdeJsonError , } , GenericError { msg : String , } , StdFormatError { err : fmt :: Error , } , CalledTemplateError { name : String , err : Box < Error > , line : usize , column : usize , } , CalledFormatterError { name : String , err : Box < Error > , line : usize , column : usize , } , # [doc (hidden)] __NonExhaustive , }
};
}

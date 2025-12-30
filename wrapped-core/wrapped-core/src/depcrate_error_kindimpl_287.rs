// Generated macro for impl_287 (impl)
macro_rules! Depcrate_error_kindimpl_287 {
() => {
// Module: crate::error::kind
// Provides: {"impl_287"}
// Dependencies: {}
impl fmt :: Display for ErrorKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: ErrorKind :: * ; match * self { Custom (ref s) => s . fmt (f) , DuplicateField (ref field) => write ! (f , "Duplicate field `{}`" , field) , MissingField (ref field) => write ! (f , "Missing field `{}`" , field) , UnknownField (ref field) => field . fmt (f) , UnsupportedShape { ref observed , ref expected , } => { write ! (f , "Unsupported shape `{}`" , observed) ? ; if let Some (expected) = & expected { write ! (f , ". Expected {}." , expected) ? ; } Ok (()) } UnexpectedFormat (ref format) => write ! (f , "Unexpected meta-item format `{}`" , format) , UnexpectedType (ref ty) => write ! (f , "Unexpected type `{}`" , ty) , UnknownValue (ref val) => val . fmt (f) , TooFewItems (ref min) => write ! (f , "Too few items: Expected at least {}" , min) , TooManyItems (ref max) => write ! (f , "Too many items: Expected no more than {}" , max) , Multiple (ref items) if items . len () == 1 => items [0] . fmt (f) , Multiple (ref items) => { write ! (f , "Multiple errors: (") ? ; write_delimited (f , items , ", ") ? ; write ! (f , ")") } __NonExhaustive => unreachable ! () , } } }
};
}

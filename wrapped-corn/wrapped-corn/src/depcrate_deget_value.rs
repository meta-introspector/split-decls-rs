// Generated macro for get_value (macro)
macro_rules! Depcrate_deget_value {
() => {
// Module: crate::de
// Provides: {"get_value"}
// Dependencies: {}
macro_rules ! get_value { ($ self : ident) => { match $ self . value . take () { Some (val) => Ok (val) , None => Err (Error :: DeserializationError (String :: from ("Deserializer value unexpectedly `None`" ,))) , } ? } ; }
};
}

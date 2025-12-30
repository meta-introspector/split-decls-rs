// Generated macro for impl_1991 (impl)
macro_rules! Depcrate_resultimpl_1991 {
() => {
// Module: crate::result
// Provides: {"impl_1991"}
// Dependencies: {}
impl fmt :: Display for DeserializeFieldError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref field_name) = self . field_name { write ! (f , "Error deserializing field '{}': {}" , field_name , self . error) } else { write ! (f , "Error deserializing field: {}" , self . error) } } }
};
}

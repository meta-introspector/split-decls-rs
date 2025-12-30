// Generated macro for impl_67 (impl)
macro_rules! Depcrate_deserializerimpl_67 {
() => {
// Module: crate::deserializer
// Provides: {"impl_67"}
// Dependencies: {}
impl fmt :: Display for DeserializeError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (field) = self . field { write ! (f , "field {}: {}" , field , self . kind) } else { write ! (f , "{}" , self . kind) } } }
};
}

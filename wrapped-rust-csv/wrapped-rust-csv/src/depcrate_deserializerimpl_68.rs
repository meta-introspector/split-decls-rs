// Generated macro for impl_68 (impl)
macro_rules! Depcrate_deserializerimpl_68 {
() => {
// Module: crate::deserializer
// Provides: {"impl_68"}
// Dependencies: {}
impl fmt :: Display for DeserializeErrorKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: DeserializeErrorKind :: * ; match * self { Message (ref msg) => write ! (f , "{}" , msg) , Unsupported (ref which) => { write ! (f , "unsupported deserializer method: {}" , which) } UnexpectedEndOfRow => write ! (f , "{}" , self . description ()) , InvalidUtf8 (ref err) => err . fmt (f) , ParseBool (ref err) => err . fmt (f) , ParseInt (ref err) => err . fmt (f) , ParseFloat (ref err) => err . fmt (f) , } } }
};
}

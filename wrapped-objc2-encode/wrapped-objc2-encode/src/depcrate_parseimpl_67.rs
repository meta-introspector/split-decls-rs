// Generated macro for impl_67 (impl)
macro_rules! Depcrate_parseimpl_67 {
() => {
// Module: crate::parse
// Provides: {"impl_67"}
// Dependencies: {}
impl fmt :: Display for ErrorKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: UnexpectedEnd => write ! (f , "unexpected end") , Self :: Unknown (b) => { write ! (f , "unknown encoding character {}" , * b as char) } Self :: UnknownAfterComplex (b) => { write ! (f , "unknown encoding character {} after complex" , * b as char ,) } Self :: ExpectedInteger => write ! (f , "expected integer") , Self :: IntegerTooLarge => write ! (f , "integer too large") , Self :: WrongEndArray => write ! (f , "expected array to be closed") , Self :: WrongEndContainer (kind) => { write ! (f , "expected {kind} to be closed") } Self :: InvalidIdentifier (kind) => { write ! (f , "got invalid identifier in {kind}") } Self :: NotAllConsumed => { write ! (f , "remaining contents after parsing") } } } }
};
}

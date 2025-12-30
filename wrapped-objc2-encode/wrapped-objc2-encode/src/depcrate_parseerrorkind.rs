// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_parseErrorKind {
() => {
// Module: crate::parse
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Hash)] pub (crate) enum ErrorKind { UnexpectedEnd , Unknown (u8) , UnknownAfterComplex (u8) , ExpectedInteger , IntegerTooLarge , WrongEndArray , WrongEndContainer (ContainerKind) , InvalidIdentifier (ContainerKind) , NotAllConsumed , }
};
}

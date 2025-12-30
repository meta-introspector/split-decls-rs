// Generated macro for unexpected_eof_to_unexpected_length_of_input (function)
macro_rules! Depcrate_deunexpected_eof_to_unexpected_length_of_input {
() => {
// Module: crate::de
// Provides: {"unexpected_eof_to_unexpected_length_of_input"}
// Dependencies: {}
fn unexpected_eof_to_unexpected_length_of_input (e : Error) -> Error { if e . kind () == ErrorKind :: UnexpectedEof { Error :: new (ErrorKind :: InvalidData , ERROR_UNEXPECTED_LENGTH_OF_INPUT) } else { e } }
};
}

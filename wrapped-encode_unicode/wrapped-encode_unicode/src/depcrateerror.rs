// Generated macro for error (module)
macro_rules! Depcrateerror {
() => {
// Module: crate
// Provides: {"error"}
// Dependencies: {}
pub mod error { # ! [doc = " Errors returned by various conversion methods in this crate."] pub use crate :: errors :: { FromStrError , EmptyStrError } ; pub use crate :: errors :: { CodepointError , NonAsciiError , NonBmpError } ; pub use crate :: errors :: { Utf8Error , Utf8ErrorKind } ; pub use crate :: errors :: { Utf16SliceError , Utf16ArrayError , Utf16TupleError } ; pub use crate :: errors :: { Utf16FirstUnitError , Utf16PairError } ; }
};
}

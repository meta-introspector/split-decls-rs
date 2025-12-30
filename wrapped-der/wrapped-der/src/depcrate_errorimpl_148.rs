// Generated macro for impl_148 (impl)
macro_rules! Depcrate_errorimpl_148 {
() => {
// Module: crate::error
// Provides: {"impl_148"}
// Dependencies: {}
impl Error { # [doc = " Create a new [`Error`]."] pub const fn new (kind : ErrorKind , position : Length) -> Error { Error { kind , position : Some (position) , } } # [doc = " Create a new [`Error`], without known position."] pub (crate) const fn from_kind (kind : ErrorKind) -> Error { Error { kind , position : None , } } # [doc = " Create a new [`ErrorKind::Incomplete`] for the given length."] # [doc = ""] # [doc = " Computes the expected len as being one greater than `actual_len`."] pub fn incomplete (actual_len : Length) -> Self { match actual_len + Length :: ONE { Ok (expected_len) => ErrorKind :: Incomplete { expected_len , actual_len , } . at (actual_len) , Err (err) => err . kind () . at (actual_len) , } } # [doc = " Get the [`ErrorKind`] which occurred."] pub fn kind (self) -> ErrorKind { self . kind } # [doc = " Get the position inside of the message where the error occurred."] pub fn position (self) -> Option < Length > { self . position } # [doc = " For errors occurring inside of a nested message, extend the position"] # [doc = " count by the location where the nested message occurs."] pub (crate) fn nested (self , nested_position : Length) -> Self { let position = (nested_position + self . position . unwrap_or_default ()) . ok () ; Self { kind : self . kind , position , } } }
};
}

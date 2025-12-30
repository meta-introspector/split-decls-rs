// Generated macro for impl_89 (impl)
macro_rules! Depcrate_errorimpl_89 {
() => {
// Module: crate::error
// Provides: {"impl_89"}
// Dependencies: {}
impl ErrorKind { # [doc = " Return the position for this error, if one exists."] # [doc = ""] # [doc = " This is a convenience function that permits callers to easily access"] # [doc = " the position on an error without doing case analysis on `ErrorKind`."] pub fn position (& self) -> Option < & Position > { match * self { ErrorKind :: Utf8 { ref pos , .. } => pos . as_ref () , ErrorKind :: UnequalLengths { ref pos , .. } => pos . as_ref () , ErrorKind :: Deserialize { ref pos , .. } => pos . as_ref () , _ => None , } } }
};
}

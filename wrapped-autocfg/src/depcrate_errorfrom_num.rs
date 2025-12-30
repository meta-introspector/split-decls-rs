// Generated macro for from_num (function)
macro_rules! Depcrate_errorfrom_num {
() => {
// Module: crate::error
// Provides: {"from_num"}
// Dependencies: {}
pub fn from_num (e : num :: ParseIntError) -> Error { Error { kind : ErrorKind :: Num (e) , } }
};
}

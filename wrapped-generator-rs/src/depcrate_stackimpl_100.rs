// Generated macro for impl_100 (impl)
macro_rules! Depcrate_stackimpl_100 {
() => {
// Module: crate::stack
// Provides: {"impl_100"}
// Dependencies: {}
impl Display for StackError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match * self { StackError :: ExceedsMaximumSize (size) => write ! (fmt , "Requested more than max size of {size} bytes for a stack") , StackError :: IoError (ref e) => e . fmt (fmt) , } } }
};
}

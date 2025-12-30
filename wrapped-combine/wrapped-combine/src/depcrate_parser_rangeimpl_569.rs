// Generated macro for impl_569 (impl)
macro_rules! Depcrate_parser_rangeimpl_569 {
() => {
// Module: crate::parser::range
// Provides: {"impl_569"}
// Dependencies: {}
impl From < Option < usize > > for TakeRange { fn from (opt : Option < usize >) -> TakeRange { match opt { Some (i) => TakeRange :: Found (i) , None => TakeRange :: NotFound (0) , } } }
};
}

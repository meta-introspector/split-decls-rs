// Generated macro for impl_wrap_char (macro)
macro_rules! Depcrate_arbitrary__alloc_charimpl_wrap_char {
() => {
// Module: crate::arbitrary::_alloc::char
// Provides: {"impl_wrap_char"}
// Dependencies: {}
macro_rules ! impl_wrap_char { ($ type : ty , $ mapper : expr) => { arbitrary ! ($ type , SMapped < char , Self >; static_map (any ::< char > () , $ mapper)) ; } ; }
};
}

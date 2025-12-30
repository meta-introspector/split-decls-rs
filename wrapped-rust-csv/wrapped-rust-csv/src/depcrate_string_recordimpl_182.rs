// Generated macro for impl_182 (impl)
macro_rules! Depcrate_string_recordimpl_182 {
() => {
// Module: crate::string_record
// Provides: {"impl_182"}
// Dependencies: {}
impl fmt :: Debug for StringRecord { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let fields : Vec < & str > = self . iter () . collect () ; write ! (f , "StringRecord({:?})" , fields) } }
};
}

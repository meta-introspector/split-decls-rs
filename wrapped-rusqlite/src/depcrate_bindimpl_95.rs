// Generated macro for impl_95 (impl)
macro_rules! Depcrate_bindimpl_95 {
() => {
// Module: crate::bind
// Provides: {"impl_95"}
// Dependencies: {}
impl BindIndex for & '_ str { fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { match stmt . parameter_index (self) ? { Some (idx) => Ok (idx) , None => Err (Error :: InvalidParameterName (self . to_string ())) , } } }
};
}

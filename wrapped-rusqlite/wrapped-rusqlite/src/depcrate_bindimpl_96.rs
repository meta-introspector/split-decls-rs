// Generated macro for impl_96 (impl)
macro_rules! Depcrate_bindimpl_96 {
() => {
// Module: crate::bind
// Provides: {"impl_96"}
// Dependencies: {}
# [doc = " C-string literal to avoid alloc"] impl BindIndex for & CStr { fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { let r = unsafe { ffi :: sqlite3_bind_parameter_index (stmt . ptr () , self . as_ptr ()) } ; match r { 0 => Err (Error :: InvalidParameterName (self . to_string_lossy () . to_string () ,)) , i => Ok (i as usize) , } } }
};
}

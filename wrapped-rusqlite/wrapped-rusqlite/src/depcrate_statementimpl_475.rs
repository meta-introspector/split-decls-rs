// Generated macro for impl_475 (impl)
macro_rules! Depcrate_statementimpl_475 {
() => {
// Module: crate::statement
// Provides: {"impl_475"}
// Dependencies: {}
impl fmt :: Debug for Statement < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let sql = if self . stmt . is_null () { Ok ("") } else { self . stmt . sql () . unwrap () . to_str () } ; f . debug_struct ("Statement") . field ("conn" , self . conn) . field ("stmt" , & self . stmt) . field ("sql" , & sql) . finish () } }
};
}

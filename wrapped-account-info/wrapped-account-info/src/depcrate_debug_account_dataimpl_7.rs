// Generated macro for impl_7 (impl)
macro_rules! Depcrate_debug_account_dataimpl_7 {
() => {
// Module: crate::debug_account_data
// Provides: {"impl_7"}
// Dependencies: {}
impl fmt :: Debug for Hex < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for & byte in self . 0 { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}

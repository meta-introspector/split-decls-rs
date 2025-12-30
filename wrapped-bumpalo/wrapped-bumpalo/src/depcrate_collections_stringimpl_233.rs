// Generated macro for impl_233 (impl)
macro_rules! Depcrate_collections_stringimpl_233 {
() => {
// Module: crate::collections::string
// Provides: {"impl_233"}
// Dependencies: {}
impl < 'bump > fmt :: Write for String < 'bump > { # [inline] fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_str (s) ; Ok (()) } # [inline] fn write_char (& mut self , c : char) -> fmt :: Result { self . push (c) ; Ok (()) } }
};
}

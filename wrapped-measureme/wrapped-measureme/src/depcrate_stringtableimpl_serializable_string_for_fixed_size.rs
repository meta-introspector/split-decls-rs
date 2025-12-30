// Generated macro for impl_serializable_string_for_fixed_size (macro)
macro_rules! Depcrate_stringtableimpl_serializable_string_for_fixed_size {
() => {
// Module: crate::stringtable
// Provides: {"impl_serializable_string_for_fixed_size"}
// Dependencies: {}
macro_rules ! impl_serializable_string_for_fixed_size { ($ n : expr) => { impl <'a > SerializableString for [StringComponent <'a >; $ n] { # [inline (always)] fn serialized_size (& self) -> usize { (& self [..]) . serialized_size () } # [inline (always)] fn serialize (& self , bytes : & mut [u8]) { (& self [..]) . serialize (bytes) ; } } } ; }
};
}

// Generated macro for impl_111 (impl)
macro_rules! Depcrate_paserkimpl_111 {
() => {
// Module: crate::paserk
// Provides: {"impl_111"}
// Dependencies: {}
impl PartialEq < Id > for Id { fn eq (& self , other : & Id) -> bool { use subtle :: ConstantTimeEq ; (self . header . as_bytes () . ct_eq (other . header . as_bytes ()) & self . identifier . as_bytes () . ct_eq (other . identifier . as_bytes ())) . into () } }
};
}

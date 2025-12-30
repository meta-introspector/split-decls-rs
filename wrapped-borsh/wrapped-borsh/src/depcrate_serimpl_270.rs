// Generated macro for impl_270 (impl)
macro_rules! Depcrate_serimpl_270 {
() => {
// Module: crate::ser
// Provides: {"impl_270"}
// Dependencies: {}
impl < T > BorshSerialize for [T] where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { writer . write_all (& (u32 :: try_from (self . len ()) . map_err (| _ | ErrorKind :: InvalidData) ?) . to_le_bytes () ,) ? ; serialize_slice (self , writer) } }
};
}

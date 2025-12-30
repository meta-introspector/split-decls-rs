// Generated macro for impl_279 (impl)
macro_rules! Depcrate_serimpl_279 {
() => {
// Module: crate::ser
// Provides: {"impl_279"}
// Dependencies: {}
impl < T > BorshSerialize for VecDeque < T > where T : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < T > () ? ; writer . write_all (& (u32 :: try_from (self . len ()) . map_err (| _ | ErrorKind :: InvalidData) ?) . to_le_bytes () ,) ? ; let slices = self . as_slices () ; serialize_slice (slices . 0 , writer) ? ; serialize_slice (slices . 1 , writer) } }
};
}

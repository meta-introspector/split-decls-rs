// Generated macro for impl_61 (impl)
macro_rules! Depcrate_versionimpl_61 {
() => {
// Module: crate::version
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > Decode < 'a > for Version { type Error = der :: Error ; fn decode < R : Reader < 'a > > (reader : & mut R) -> der :: Result < Self > { Version :: try_from (u8 :: decode (reader) ?) . map_err (| _ | reader . error (Self :: TAG . value_error ())) } }
};
}

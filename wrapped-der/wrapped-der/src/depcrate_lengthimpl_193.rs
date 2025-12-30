// Generated macro for impl_193 (impl)
macro_rules! Depcrate_lengthimpl_193 {
() => {
// Module: crate::length
// Provides: {"impl_193"}
// Dependencies: {}
impl TryFrom < Length > for usize { type Error = Error ; fn try_from (len : Length) -> Result < usize > { len . inner . try_into () . map_err (| _ | ErrorKind :: Overflow . into ()) } }
};
}

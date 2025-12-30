// Generated macro for impl_466 (impl)
macro_rules! Depcrate_provider_fieldsimpl_466 {
() => {
// Module: crate::provider::fields
// Provides: {"impl_466"}
// Dependencies: {}
impl TryFrom < (FieldSymbol , usize) > for Field { type Error = Error ; fn try_from (input : (FieldSymbol , usize)) -> Result < Self , Self :: Error > { let length = FieldLength :: from_idx (input . 1 as u8) . map_err (| _ | Self :: Error :: InvalidLength (input . 0)) ? ; Ok (Self { symbol : input . 0 , length , }) } }
};
}

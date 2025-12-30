// Generated macro for impl_94 (impl)
macro_rules! Depcrate_uts46impl_94 {
() => {
// Module: crate::uts46
// Provides: {"impl_94"}
// Dependencies: {}
impl From < crate :: punycode :: PunycodeEncodeError > for ProcessingError { fn from (_ : crate :: punycode :: PunycodeEncodeError) -> Self { unreachable ! ("Punycode overflows should not be possible due to PUNYCODE_ENCODE_MAX_INPUT_LENGTH") ; } }
};
}

// Generated macro for impl_639 (impl)
macro_rules! Depcrate_types_scalarsimpl_639 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_639"}
// Dependencies: {}
impl < 's , S > FromScalarValue < 's , S > for & 's str where S : TryToPrimitive < 's , Self , Error : IntoFieldError < S > > + 's , { type Error = S :: Error ; fn from_scalar_value (v : & 's S) -> Result < Self , Self :: Error > { v . try_to_primitive () } }
};
}

// Generated macro for build_conversion_impl (macro)
macro_rules! Depcrate_data_valuebuild_conversion_impl {
() => {
// Module: crate::data_value
// Provides: {"build_conversion_impl"}
// Dependencies: {}
# [doc = " Helper for creating conversion implementations for [DataValue]."] macro_rules ! build_conversion_impl { ($ rust_ty : ty , $ data_value_ty : ident , $ cranelift_ty : ident) => { impl From <$ rust_ty > for DataValue { fn from (data : $ rust_ty) -> Self { DataValue ::$ data_value_ty (data) } } impl TryInto <$ rust_ty > for DataValue { type Error = DataValueCastFailure ; fn try_into (self) -> Result <$ rust_ty , Self :: Error > { if let DataValue ::$ data_value_ty (v) = self { Ok (v) } else { Err (DataValueCastFailure :: TryInto (self . ty () , types ::$ cranelift_ty ,)) } } } } ; }
};
}

// Generated macro for get_discriminant_value (function)
macro_rules! Depcrate_tyget_discriminant_value {
() => {
// Module: crate::ty
// Provides: {"get_discriminant_value"}
// Dependencies: {}
# [doc = " Gets the value of the given variant."] pub fn get_discriminant_value (tcx : TyCtxt < '_ > , adt : AdtDef < '_ > , i : VariantIdx) -> EnumValue { let variant = & adt . variant (i) ; match variant . discr { VariantDiscr :: Explicit (id) => read_explicit_enum_value (tcx , id) . unwrap () , VariantDiscr :: Relative (x) => match adt . variant ((i . as_usize () - x as usize) . into ()) . discr { VariantDiscr :: Explicit (id) => read_explicit_enum_value (tcx , id) . unwrap () + x , VariantDiscr :: Relative (_) => EnumValue :: Unsigned (x . into ()) , } , } }
};
}

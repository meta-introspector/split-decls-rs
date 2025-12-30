// Generated macro for is_enum_variant_uninhabited_from (function)
macro_rules! Depcrate_inhabitednessis_enum_variant_uninhabited_from {
() => {
// Module: crate::inhabitedness
// Provides: {"is_enum_variant_uninhabited_from"}
// Dependencies: {}
# [doc = " Checks whether a variant is visibly uninhabited from a particular module."] pub (crate) fn is_enum_variant_uninhabited_from < 'db > (infcx : & InferCtxt < 'db > , variant : EnumVariantId , subst : GenericArgs < 'db > , target_mod : ModuleId , env : Arc < TraitEnvironment < 'db > > ,) -> bool { let _p = tracing :: info_span ! ("is_enum_variant_uninhabited_from") . entered () ; let mut uninhabited_from = UninhabitedFrom :: new (infcx , target_mod , env) ; let inhabitedness = uninhabited_from . visit_variant (variant . into () , subst) ; inhabitedness == BREAK_VISIBLY_UNINHABITED }
};
}

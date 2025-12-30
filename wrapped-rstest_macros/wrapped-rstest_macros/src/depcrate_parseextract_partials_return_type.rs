// Generated macro for extract_partials_return_type (function)
macro_rules! Depcrate_parseextract_partials_return_type {
() => {
// Module: crate::parse
// Provides: {"extract_partials_return_type"}
// Dependencies: {}
pub (crate) fn extract_partials_return_type (item_fn : & mut ItemFn ,) -> Result < Vec < (usize , syn :: Type) > , ErrorsVec > { let mut partials_type_extractor = PartialsTypeFunctionExtractor :: default () ; partials_type_extractor . visit_item_fn_mut (item_fn) ; partials_type_extractor . take () }
};
}

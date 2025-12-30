// Generated macro for extract_default_return_type (function)
macro_rules! Depcrate_parseextract_default_return_type {
() => {
// Module: crate::parse
// Provides: {"extract_default_return_type"}
// Dependencies: {}
pub (crate) fn extract_default_return_type (item_fn : & mut ItemFn ,) -> Result < Option < syn :: Type > , ErrorsVec > { struct DefaultTypeBuilder ; impl AttrBuilder < ItemFn > for DefaultTypeBuilder { type Out = syn :: Type ; fn build (attr : syn :: Attribute , _extra : & ItemFn) -> syn :: Result < Self :: Out > { attr . parse_args :: < syn :: Type > () } } impl Validator < syn :: ItemFn > for DefaultTypeBuilder { } let mut extractor = JustOnceFnAttributeExtractor :: < DefaultTypeBuilder > :: new (FixtureModifiers :: DEFAULT_RET_ATTR) ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}

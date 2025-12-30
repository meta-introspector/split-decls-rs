// Generated macro for extract_defaults (function)
macro_rules! Depcrate_parseextract_defaults {
() => {
// Module: crate::parse
// Provides: {"extract_defaults"}
// Dependencies: {}
pub (crate) fn extract_defaults (item_fn : & mut ItemFn) -> Result < Vec < ArgumentValue > , ErrorsVec > { struct DefaultBuilder ; impl AttrBuilder < Pat > for DefaultBuilder { type Out = ArgumentValue ; fn build (attr : syn :: Attribute , name : & Pat) -> syn :: Result < Self :: Out > { attr . parse_args :: < syn :: Expr > () . map (| e | ArgumentValue :: new (name . clone () , e)) } } impl Validator < syn :: FnArg > for DefaultBuilder { } let mut extractor = JustOnceFnArgAttributeExtractor :: < DefaultBuilder > :: new ("default") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}

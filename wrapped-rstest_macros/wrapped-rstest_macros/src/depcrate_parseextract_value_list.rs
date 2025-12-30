// Generated macro for extract_value_list (function)
macro_rules! Depcrate_parseextract_value_list {
() => {
// Module: crate::parse
// Provides: {"extract_value_list"}
// Dependencies: {}
pub (crate) fn extract_value_list (item_fn : & mut ItemFn) -> Result < Vec < ValueList > , ErrorsVec > { struct ValueListBuilder ; impl AttrBuilder < Pat > for ValueListBuilder { type Out = ValueList ; fn build (attr : syn :: Attribute , extra : & Pat) -> syn :: Result < Self :: Out > { attr . parse_args :: < Expressions > () . map (| v | ValueList { arg : extra . clone () , values : v . take () . into_iter () . map (| e | e . into ()) . collect () , }) } } impl Validator < FnArg > for ValueListBuilder { } let mut extractor = JustOnceFnArgAttributeExtractor :: < ValueListBuilder > :: new ("values") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}

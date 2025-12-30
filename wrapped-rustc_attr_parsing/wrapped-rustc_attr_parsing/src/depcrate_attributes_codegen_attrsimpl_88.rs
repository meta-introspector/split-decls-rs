// Generated macro for impl_88 (impl)
macro_rules! Depcrate_attributes_codegen_attrsimpl_88 {
() => {
// Module: crate::attributes::codegen_attrs
// Provides: {"impl_88"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for OptimizeParser { const PATH : & [Symbol] = & [sym :: optimize] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: WarnButFutureError ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Closure) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Inherent)) ,]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["size" , "speed" , "none"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; let res = match single . meta_item () . and_then (| i | i . path () . word () . map (| i | i . name)) { Some (sym :: size) => OptimizeAttr :: Size , Some (sym :: speed) => OptimizeAttr :: Speed , Some (sym :: none) => OptimizeAttr :: DoNotOptimize , _ => { cx . expected_specific_argument (single . span () , & [sym :: size , sym :: speed , sym :: none]) ; OptimizeAttr :: Default } } ; Some (AttributeKind :: Optimize (res , cx . attr_span)) } }
};
}

// Generated macro for impl_297 (impl)
macro_rules! Depcrate_attributes_rustc_internalimpl_297 {
() => {
// Module: crate::attributes::rustc_internal
// Provides: {"impl_297"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for RustcLayoutScalarValidRangeStart { const PATH : & 'static [Symbol] = & [sym :: rustc_layout_scalar_valid_range_start] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["start"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { parse_single_integer (cx , args) . map (| n | AttributeKind :: RustcLayoutScalarValidRangeStart (Box :: new (n) , cx . attr_span)) } }
};
}

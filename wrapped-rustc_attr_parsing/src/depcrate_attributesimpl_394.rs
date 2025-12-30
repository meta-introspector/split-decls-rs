// Generated macro for impl_394 (impl)
macro_rules! Depcrate_attributesimpl_394 {
() => {
// Module: crate::attributes
// Provides: {"impl_394"}
// Dependencies: {}
impl < T : SingleAttributeParser < S > , S : Stage > AttributeParser < S > for Single < T , S > { const ATTRIBUTES : AcceptMapping < Self , S > = & [(T :: PATH , < T as SingleAttributeParser < S > > :: TEMPLATE , | group : & mut Single < T , S > , cx , args | { if let Some (pa) = T :: convert (cx , args) { match T :: ATTRIBUTE_ORDER { AttributeOrder :: KeepInnermost => { if let Some ((_ , unused)) = group . 1 { T :: ON_DUPLICATE . exec :: < T > (cx , cx . attr_span , unused) ; return ; } } AttributeOrder :: KeepOutermost => { if let Some ((_ , used)) = group . 1 { T :: ON_DUPLICATE . exec :: < T > (cx , used , cx . attr_span) ; } } } group . 1 = Some ((pa , cx . attr_span)) ; } } ,)] ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TYPE : AttributeType = T :: TYPE ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { Some (self . 1 ? . 0) } }
};
}

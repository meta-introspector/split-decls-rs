// Generated macro for impl_401 (impl)
macro_rules! Depcrate_attributesimpl_401 {
() => {
// Module: crate::attributes
// Provides: {"impl_401"}
// Dependencies: {}
impl < T : NoArgsAttributeParser < S > , S : Stage > SingleAttributeParser < S > for WithoutArgs < T , S > { const PATH : & [Symbol] = T :: PATH ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = T :: ON_DUPLICATE ; const ALLOWED_TARGETS : AllowedTargets = T :: ALLOWED_TARGETS ; const TEMPLATE : AttributeTemplate = template ! (Word) ; const TYPE : AttributeType = T :: TYPE ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { if let Err (span) = args . no_args () { cx . expected_no_args (span) ; } Some (T :: CREATE (cx . attr_span)) } }
};
}

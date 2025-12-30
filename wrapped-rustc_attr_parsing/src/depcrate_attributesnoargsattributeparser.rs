// Generated macro for NoArgsAttributeParser (trait)
macro_rules! Depcrate_attributesNoArgsAttributeParser {
() => {
// Module: crate::attributes
// Provides: {"NoArgsAttributeParser"}
// Dependencies: {}
# [doc = " An even simpler version of [`SingleAttributeParser`]:"] # [doc = " now automatically check that there are no arguments provided to the attribute."] # [doc = ""] # [doc = " [`WithoutArgs<T> where T: NoArgsAttributeParser`](WithoutArgs) implements [`SingleAttributeParser`]."] pub (crate) trait NoArgsAttributeParser < S : Stage > : 'static { const PATH : & [Symbol] ; const ON_DUPLICATE : OnDuplicate < S > ; const ALLOWED_TARGETS : AllowedTargets ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " Create the [`AttributeKind`] given attribute's [`Span`]."] const CREATE : fn (Span) -> AttributeKind ; }
};
}

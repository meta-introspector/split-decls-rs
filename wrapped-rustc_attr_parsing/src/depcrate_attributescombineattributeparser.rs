// Generated macro for CombineAttributeParser (trait)
macro_rules! Depcrate_attributesCombineAttributeParser {
() => {
// Module: crate::attributes
// Provides: {"CombineAttributeParser"}
// Dependencies: {}
# [doc = " Alternative to [`AttributeParser`] that automatically handles state management."] # [doc = " If multiple attributes appear on an element, combines the values of each into a"] # [doc = " [`ThinVec`]."] # [doc = " [`Combine<T> where T: CombineAttributeParser`](Combine) implements [`AttributeParser`]."] # [doc = ""] # [doc = " [`CombineAttributeParser`] can only convert a single kind of attribute, and cannot combine multiple"] # [doc = " attributes together like is necessary for `#[stable()]` and `#[unstable()]` for example."] pub (crate) trait CombineAttributeParser < S : Stage > : 'static { const PATH : & [rustc_span :: Symbol] ; type Item ; # [doc = " A function that converts individual items (of type [`Item`](Self::Item)) into the final attribute."] # [doc = ""] # [doc = " For example, individual representations fomr `#[repr(...)]` attributes into an `AttributeKind::Repr(x)`,"] # [doc = "  where `x` is a vec of these individual reprs."] const CONVERT : ConvertFn < Self :: Item > ; const ALLOWED_TARGETS : AllowedTargets ; # [doc = " The template this attribute parser should implement. Used for diagnostics."] const TEMPLATE : AttributeTemplate ; const TYPE : AttributeType = AttributeType :: Normal ; # [doc = " Converts a single syntactical attribute to a number of elements of the semantic attribute, or [`AttributeKind`]"] fn extend < 'c > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> impl IntoIterator < Item = Self :: Item > + 'c ; }
};
}

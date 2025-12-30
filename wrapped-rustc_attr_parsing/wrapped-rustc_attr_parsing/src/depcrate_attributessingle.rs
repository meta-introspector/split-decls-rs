// Generated macro for Single (struct)
macro_rules! Depcrate_attributesSingle {
() => {
// Module: crate::attributes
// Provides: {"Single"}
// Dependencies: {}
# [doc = " Use in combination with [`SingleAttributeParser`]."] # [doc = " `Single<T: SingleAttributeParser>` implements [`AttributeParser`]."] pub (crate) struct Single < T : SingleAttributeParser < S > , S : Stage > (PhantomData < (S , T) > , Option < (AttributeKind , Span) > ,) ;
};
}

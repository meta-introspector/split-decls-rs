// Generated macro for parenthesize_impl_trait (function)
macro_rules! Depcrate_lifetimeparenthesize_impl_trait {
() => {
// Module: crate::lifetime
// Provides: {"parenthesize_impl_trait"}
// Dependencies: {}
fn parenthesize_impl_trait (elem : & mut Type , paren_span : Span) { if let Type :: ImplTrait (_) = * elem { let placeholder = Type :: Verbatim (TokenStream :: new ()) ; * elem = Type :: Paren (TypeParen { paren_token : token :: Paren (paren_span) , elem : Box :: new (mem :: replace (elem , placeholder)) , }) ; } }
};
}

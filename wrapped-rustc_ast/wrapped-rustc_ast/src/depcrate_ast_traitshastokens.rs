// Generated macro for HasTokens (trait)
macro_rules! Depcrate_ast_traitsHasTokens {
() => {
// Module: crate::ast_traits
// Provides: {"HasTokens"}
// Dependencies: {}
# [doc = " A trait for AST nodes having (or not having) collected tokens."] pub trait HasTokens { fn tokens (& self) -> Option < & LazyAttrTokenStream > ; fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > ; }
};
}

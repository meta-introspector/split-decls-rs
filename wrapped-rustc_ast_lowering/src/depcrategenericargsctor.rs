// Generated macro for GenericArgsCtor (struct)
macro_rules! DepcrateGenericArgsCtor {
() => {
// Module: crate
// Provides: {"GenericArgsCtor"}
// Dependencies: {}
# [doc = " Helper struct for the delayed construction of [`hir::GenericArgs`]."] struct GenericArgsCtor < 'hir > { args : SmallVec < [hir :: GenericArg < 'hir > ; 4] > , constraints : & 'hir [hir :: AssocItemConstraint < 'hir >] , parenthesized : hir :: GenericArgsParentheses , span : Span , }
};
}

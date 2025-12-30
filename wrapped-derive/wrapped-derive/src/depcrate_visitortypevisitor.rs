// Generated macro for TypeVisitor (struct)
macro_rules! Depcrate_visitorTypeVisitor {
() => {
// Module: crate::visitor
// Provides: {"TypeVisitor"}
// Dependencies: {}
struct TypeVisitor < 'a > { # [doc = " The type parameters in scope"] typarams : & 'a HashSet < Ident > , # [doc = " Whether we found a type parameter"] found_typarams : bool , # [doc = " Whether we found a non-'static lifetime parameter"] found_lifetimes : bool , }
};
}

// Generated macro for UnionItem (struct)
macro_rules! Depcrate_argsUnionItem {
() => {
// Module: crate::args
// Provides: {"UnionItem"}
// Dependencies: {}
# [derive (FromVariant)] # [darling (attributes (graphql))] pub struct UnionItem { pub ident : Ident , pub fields : Fields < syn :: Type > , # [darling (default)] pub flatten : bool , }
};
}

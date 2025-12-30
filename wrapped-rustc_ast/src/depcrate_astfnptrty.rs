// Generated macro for FnPtrTy (struct)
macro_rules! Depcrate_astFnPtrTy {
() => {
// Module: crate::ast
// Provides: {"FnPtrTy"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FnPtrTy { pub safety : Safety , pub ext : Extern , pub generic_params : ThinVec < GenericParam > , pub decl : Box < FnDecl > , # [doc = " Span of the `[unsafe] [extern] fn(...) -> ...` part, i.e. everything"] # [doc = " after the generic params (if there are any, e.g. `for<'a>`)."] pub decl_span : Span , }
};
}

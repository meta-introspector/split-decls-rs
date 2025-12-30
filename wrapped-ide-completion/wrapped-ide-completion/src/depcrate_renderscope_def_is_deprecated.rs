// Generated macro for scope_def_is_deprecated (function)
macro_rules! Depcrate_renderscope_def_is_deprecated {
() => {
// Module: crate::render
// Provides: {"scope_def_is_deprecated"}
// Dependencies: {}
fn scope_def_is_deprecated (ctx : & RenderContext < '_ > , resolution : ScopeDef) -> bool { match resolution { ScopeDef :: ModuleDef (it) => ctx . is_deprecated_assoc_item (it) , ScopeDef :: GenericParam (it) => ctx . is_deprecated (it) , ScopeDef :: AdtSelfType (it) => ctx . is_deprecated (it) , _ => false , } }
};
}

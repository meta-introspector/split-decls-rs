// Generated macro for to_def_impls (macro)
macro_rules! Depcrate_semanticsto_def_impls {
() => {
// Module: crate::semantics
// Provides: {"to_def_impls"}
// Dependencies: {}
macro_rules ! to_def_impls { ($ (($ def : path , $ ast : path , $ meth : ident)) ,* ,) => { $ (impl ToDef for $ ast { type Def = $ def ; fn to_def (sema : & SemanticsImpl <'_ >, src : InFile <& Self >) -> Option < Self :: Def > { sema . with_ctx (| ctx | ctx .$ meth (src)) . map (<$ def >:: from) } }) * } }
};
}

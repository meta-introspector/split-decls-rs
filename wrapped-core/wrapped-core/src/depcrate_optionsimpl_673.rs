// Generated macro for impl_673 (impl)
macro_rules! Depcrate_optionsimpl_673 {
() => {
// Module: crate::options
// Provides: {"impl_673"}
// Dependencies: {}
# [doc (hidden)] impl FromMeta for DefaultExpression { fn from_meta (item : & syn :: Meta) -> Result < Self > { match item { syn :: Meta :: Path (_) => Ok (DefaultExpression :: Trait { span : item . span () }) , syn :: Meta :: List (nm) => Err (Error :: unsupported_format ("list") . with_span (nm)) , syn :: Meta :: NameValue (nv) => Self :: from_expr (& nv . value) , } } fn from_expr (expr : & syn :: Expr) -> Result < Self > { Callable :: from_expr (expr) . map (Self :: Explicit) } fn from_value (value : & syn :: Lit) -> Result < Self > { Callable :: from_value (value) . map (Self :: Explicit) } }
};
}

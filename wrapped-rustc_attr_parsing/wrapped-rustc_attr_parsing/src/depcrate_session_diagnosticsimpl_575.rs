// Generated macro for impl_575 (impl)
macro_rules! Depcrate_session_diagnosticsimpl_575 {
() => {
// Module: crate::session_diagnostics
// Provides: {"impl_575"}
// Dependencies: {}
impl IncorrectReprFormatGenericCause { pub (crate) fn from_lit_kind (span : Span , kind : & ast :: LitKind , name : Symbol) -> Option < Self > { match * kind { ast :: LitKind :: Int (value , ast :: LitIntType :: Unsuffixed) => { Some (Self :: Int { span , name , value : value . get () }) } ast :: LitKind :: Str (value , _) => Some (Self :: Symbol { span , name , value }) , _ => None , } } }
};
}

// Generated macro for evar_with_default (function)
macro_rules! Depcrate_colorevar_with_default {
() => {
// Module: crate::color
// Provides: {"evar_with_default"}
// Dependencies: {}
fn evar_with_default < 'a > (name : & str , default : & 'a str) -> Cow < 'a , OsStr > { std :: env :: var_os (name) . map (Cow :: from) . unwrap_or_else (| | Cow :: Borrowed (OsStr :: new (default))) }
};
}

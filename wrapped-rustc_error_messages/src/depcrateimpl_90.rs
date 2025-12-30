// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl From < DiagArgValue > for FluentValue < 'static > { fn from (val : DiagArgValue) -> Self { match val { DiagArgValue :: Str (s) => From :: from (s) , DiagArgValue :: Number (n) => From :: from (n) , DiagArgValue :: StrListSepByAnd (l) => fluent_value_from_str_list_sep_by_and (l) , } } }
};
}

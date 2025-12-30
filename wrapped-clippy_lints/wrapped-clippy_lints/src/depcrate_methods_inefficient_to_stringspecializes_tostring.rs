// Generated macro for specializes_tostring (function)
macro_rules! Depcrate_methods_inefficient_to_stringspecializes_tostring {
() => {
// Module: crate::methods::inefficient_to_string
// Provides: {"specializes_tostring"}
// Dependencies: {}
# [doc = " Returns whether `ty` specializes `ToString`."] # [doc = " Currently, these are `str`, `String`, and `Cow<'_, str>`."] fn specializes_tostring (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { if let ty :: Str = ty . kind () { return true ; } if ty . is_lang_item (cx , hir :: LangItem :: String) { return true ; } if let ty :: Adt (adt , args) = ty . kind () { cx . tcx . is_diagnostic_item (sym :: Cow , adt . did ()) && args . type_at (1) . is_str () } else { false } }
};
}

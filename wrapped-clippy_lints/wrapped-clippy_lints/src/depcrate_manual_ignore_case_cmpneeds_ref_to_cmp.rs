// Generated macro for needs_ref_to_cmp (function)
macro_rules! Depcrate_manual_ignore_case_cmpneeds_ref_to_cmp {
() => {
// Module: crate::manual_ignore_case_cmp
// Provides: {"needs_ref_to_cmp"}
// Dependencies: {}
# [doc = " Returns true if the type needs to be dereferenced to be compared"] fn needs_ref_to_cmp (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . is_char () || * ty . kind () == ty :: Uint (UintTy :: U8) || ty . is_diag_item (cx , sym :: Vec) || ty . is_lang_item (cx , LangItem :: String) }
};
}

// Generated macro for is_type_lang_item (function)
macro_rules! Depcrate_tyis_type_lang_item {
() => {
// Module: crate::ty
// Provides: {"is_type_lang_item"}
// Dependencies: {}
# [doc = " Checks if the type is equal to a lang item."] # [doc = ""] # [doc = " Returns `false` if the `LangItem` is not defined."] pub fn is_type_lang_item (cx : & LateContext < '_ > , ty : Ty < '_ > , lang_item : LangItem) -> bool { match ty . kind () { ty :: Adt (adt , _) => cx . tcx . lang_items () . get (lang_item) == Some (adt . did ()) , _ => false , } }
};
}

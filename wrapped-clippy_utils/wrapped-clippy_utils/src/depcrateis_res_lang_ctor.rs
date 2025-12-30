// Generated macro for is_res_lang_ctor (function)
macro_rules! Depcrateis_res_lang_ctor {
() => {
// Module: crate
// Provides: {"is_res_lang_ctor"}
// Dependencies: {}
# [doc = " Checks if a `Res` refers to a constructor of a `LangItem`"] # [doc = " For example, use this to check whether a function call or a pattern is `Some(..)`."] pub fn is_res_lang_ctor (cx : & LateContext < '_ > , res : Res , lang_item : LangItem) -> bool { if let Res :: Def (DefKind :: Ctor (..) , id) = res && let Some (lang_id) = cx . tcx . lang_items () . get (lang_item) && let Some (id) = cx . tcx . opt_parent (id) { id == lang_id } else { false } }
};
}

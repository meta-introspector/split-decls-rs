// Generated macro for is_lang_item_or_ctor (function)
macro_rules! Depcrateis_lang_item_or_ctor {
() => {
// Module: crate
// Provides: {"is_lang_item_or_ctor"}
// Dependencies: {}
# [doc = " Checks if the `DefId` matches the given `LangItem` or it's constructor."] pub fn is_lang_item_or_ctor (cx : & LateContext < '_ > , did : DefId , item : LangItem) -> bool { let did = match cx . tcx . def_kind (did) { DefKind :: Ctor (..) => cx . tcx . parent (did) , DefKind :: Variant => match cx . tcx . opt_parent (did) { Some (did) if matches ! (cx . tcx . def_kind (did) , DefKind :: Variant) => did , _ => did , } , _ => did , } ; cx . tcx . lang_items () . get (item) == Some (did) }
};
}

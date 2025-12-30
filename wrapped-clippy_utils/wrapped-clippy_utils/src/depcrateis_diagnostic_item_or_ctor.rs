// Generated macro for is_diagnostic_item_or_ctor (function)
macro_rules! Depcrateis_diagnostic_item_or_ctor {
() => {
// Module: crate
// Provides: {"is_diagnostic_item_or_ctor"}
// Dependencies: {}
# [doc = " Checks if the `DefId` matches the given diagnostic item or it's constructor."] pub fn is_diagnostic_item_or_ctor (cx : & LateContext < '_ > , did : DefId , item : Symbol) -> bool { let did = match cx . tcx . def_kind (did) { DefKind :: Ctor (..) => cx . tcx . parent (did) , DefKind :: Variant => match cx . tcx . opt_parent (did) { Some (did) if matches ! (cx . tcx . def_kind (did) , DefKind :: Variant) => did , _ => did , } , _ => did , } ; cx . tcx . is_diagnostic_item (item , did) }
};
}

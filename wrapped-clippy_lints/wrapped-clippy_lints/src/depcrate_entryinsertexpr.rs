// Generated macro for InsertExpr (struct)
macro_rules! Depcrate_entryInsertExpr {
() => {
// Module: crate::entry
// Provides: {"InsertExpr"}
// Dependencies: {}
# [doc = " Details on an expression inserting a key into a map."] # [doc = ""] # [doc = " For instance, on the following:"] # [doc = " ```ignore"] # [doc = " self.the_map.insert(\"the_key\", 3 + 4);"] # [doc = " ```"] # [doc = ""] # [doc = " - `map` will be the `self.the_map` expression"] # [doc = " - `key` will be the `\"the_key\"` expression"] # [doc = " - `value` will be the `3 + 4` expression"] struct InsertExpr < 'tcx > { # [doc = " The map into which the insertion is performed."] map : & 'tcx Expr < 'tcx > , # [doc = " The key at which to insert."] key : & 'tcx Expr < 'tcx > , # [doc = " The value to insert."] value : & 'tcx Expr < 'tcx > , }
};
}

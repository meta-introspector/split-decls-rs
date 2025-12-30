// Generated macro for expr_or_init (function)
macro_rules! Depcrateexpr_or_init {
() => {
// Module: crate
// Provides: {"expr_or_init"}
// Dependencies: {}
# [doc = " If the given expression is a local binding, find the initializer expression."] # [doc = " If that initializer expression is another local binding, find its initializer again."] # [doc = ""] # [doc = " This process repeats as long as possible (but usually no more than once). Initializer"] # [doc = " expressions with adjustments are ignored. If this is not desired, use [`find_binding_init`]"] # [doc = " instead."] # [doc = ""] # [doc = " Examples:"] # [doc = " ```no_run"] # [doc = " let abc = 1;"] # [doc = " //        ^ output"] # [doc = " let def = abc;"] # [doc = " dbg!(def);"] # [doc = " //   ^^^ input"] # [doc = ""] # [doc = " // or..."] # [doc = " let abc = 1;"] # [doc = " let def = abc + 2;"] # [doc = " //        ^^^^^^^ output"] # [doc = " dbg!(def);"] # [doc = " //   ^^^ input"] # [doc = " ```"] pub fn expr_or_init < 'a , 'b , 'tcx : 'b > (cx : & LateContext < 'tcx > , mut expr : & 'a Expr < 'b >) -> & 'a Expr < 'b > { while let Some (init) = expr . res_local_id () . and_then (| id | find_binding_init (cx , id)) . filter (| init | cx . typeck_results () . expr_adjustments (init) . is_empty ()) { expr = init ; } expr }
};
}

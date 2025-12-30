// Generated macro for let_binding_name (function)
macro_rules! Depcrate_map_unit_fnlet_binding_name {
() => {
// Module: crate::map_unit_fn
// Provides: {"let_binding_name"}
// Dependencies: {}
# [doc = " Builds a name for the let binding variable (`var_arg`)"] # [doc = ""] # [doc = " `x.field` => `x_field`"] # [doc = " `y` => `_y`"] # [doc = ""] # [doc = " Anything else will return `a`."] fn let_binding_name (cx : & LateContext < '_ > , var_arg : & hir :: Expr < '_ >) -> String { match & var_arg . kind { hir :: ExprKind :: Field (_ , _) => snippet (cx , var_arg . span , "_") . replace ('.' , "_") , hir :: ExprKind :: Path (_) => format ! ("_{}" , snippet (cx , var_arg . span , "")) , _ => "a" . to_string () , } }
};
}

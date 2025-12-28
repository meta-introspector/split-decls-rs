macro_rules! deps {
    () => {
        FindSignificantDropper!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < 'tcx > FindSignificantDropper < '_ , 'tcx > { # [doc = " Check the scrutinee of an `if let` to see if it promotes any temporary values"] # [doc = " that would change drop order in edition 2024. Specifically, it checks the value"] # [doc = " of the scrutinee itself, and also recurses into the expression to find any ref"] # [doc = " exprs (or autoref) which would promote temporaries that would be scoped to the"] # [doc = " end of this `if`."] fn check_if_let_scrutinee (& mut self , init : & 'tcx hir :: Expr < 'tcx > ,) -> ControlFlow < (Span , SmallVec < [Ty < 'tcx > ; 4] >) > { self . check_promoted_temp_with_drop (init) ? ; self . visit_expr (init) } # [doc = " Check that an expression is not a promoted temporary with a significant"] # [doc = " drop impl."] # [doc = ""] # [doc = " An expression is a promoted temporary if it has an addr taken (i.e. `&expr` or autoref)"] # [doc = " or is the scrutinee of the `if let`, *and* the expression is not a place"] # [doc = " expr, and it has a significant drop."] fn check_promoted_temp_with_drop (& self , expr : & 'tcx hir :: Expr < 'tcx > ,) -> ControlFlow < (Span , SmallVec < [Ty < 'tcx > ; 4] >) > { if expr . is_place_expr (| base | { self . cx . typeck_results () . adjustments () . get (base . hir_id) . is_some_and (| x | x . iter () . any (| adj | matches ! (adj . kind , Adjust :: Deref (_)))) }) { return ControlFlow :: Continue (()) ; } let drop_tys = extract_component_with_significant_dtor (self . cx . tcx , self . cx . typing_env () , self . cx . typeck_results () . expr_ty (expr) ,) ; if drop_tys . is_empty () { return ControlFlow :: Continue (()) ; } ControlFlow :: Break ((expr . span , drop_tys)) } }
    };
}

impl_244!();
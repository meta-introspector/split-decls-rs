macro_rules! deps {
    () => {
        OptimizedExpr!();
        OptimizedRule!();
    };
}

macro_rules! restore_on_err {
    () => {
        deps!();
        pub fn restore_on_err (rule : OptimizedRule , rules : & HashMap < String , OptimizedExpr > ,) -> OptimizedRule { let OptimizedRule { name , ty , expr } = rule ; let expr = expr . map_bottom_up (| expr | wrap_branching_exprs (expr , rules)) ; OptimizedRule { name , ty , expr } }
    };
}

restore_on_err!();
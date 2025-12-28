macro_rules! contains_self_type_expr {
    () => {
        fn contains_self_type_expr (expr : & Expr) -> bool { match expr { Expr :: Path (ExprPath { qself : Some (_) , .. }) => true , Expr :: Path (ExprPath { path , .. }) => contains_self_type_path (path) , _ => false , } }
    };
}

contains_self_type_expr!();
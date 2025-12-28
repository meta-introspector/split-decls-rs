macro_rules! contains_exterior_struct_lit {
    () => {
        # [doc = " Expressions that syntactically contain an \"exterior\" struct literal, i.e., not surrounded by any"] # [doc = " parens or other delimiters, e.g., `X { y: 1 }`, `X { y: 1 }.method()`, `foo == X { y: 1 }` and"] # [doc = " `X { y: 1 } == foo` all do, but `(X { y: 1 }) == foo` does not."] fn contains_exterior_struct_lit (value : & hir :: Expr < '_ >) -> bool { match value . kind { hir :: ExprKind :: Struct (..) => true , hir :: ExprKind :: Assign (lhs , rhs , _) | hir :: ExprKind :: AssignOp (_ , lhs , rhs) | hir :: ExprKind :: Binary (_ , lhs , rhs) => { contains_exterior_struct_lit (lhs) || contains_exterior_struct_lit (rhs) } hir :: ExprKind :: Unary (_ , x) | hir :: ExprKind :: Cast (x , _) | hir :: ExprKind :: Type (x , _) | hir :: ExprKind :: Field (x , _) | hir :: ExprKind :: Index (x , _ , _) => { contains_exterior_struct_lit (x) } hir :: ExprKind :: MethodCall (_ , receiver , ..) => { contains_exterior_struct_lit (receiver) } _ => false , } }
    };
}

contains_exterior_struct_lit!()
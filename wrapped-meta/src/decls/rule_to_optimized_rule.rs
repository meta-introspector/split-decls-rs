macro_rules! deps {
    () => {
        OptimizedRule!();
        Rule!();
        Expr!();
        OptimizedExpr!();
    };
}

macro_rules! rule_to_optimized_rule {
    () => {
        deps!();
        fn rule_to_optimized_rule (rule : Rule) -> OptimizedRule { fn to_optimized (expr : Expr) -> OptimizedExpr { match expr { Expr :: Str (string) => OptimizedExpr :: Str (string) , Expr :: Insens (string) => OptimizedExpr :: Insens (string) , Expr :: Range (start , end) => OptimizedExpr :: Range (start , end) , Expr :: Ident (ident) => OptimizedExpr :: Ident (ident) , Expr :: PeekSlice (start , end) => OptimizedExpr :: PeekSlice (start , end) , Expr :: PosPred (expr) => OptimizedExpr :: PosPred (Box :: new (to_optimized (* expr))) , Expr :: NegPred (expr) => OptimizedExpr :: NegPred (Box :: new (to_optimized (* expr))) , Expr :: Seq (lhs , rhs) => { OptimizedExpr :: Seq (Box :: new (to_optimized (* lhs)) , Box :: new (to_optimized (* rhs))) } Expr :: Choice (lhs , rhs) => { OptimizedExpr :: Choice (Box :: new (to_optimized (* lhs)) , Box :: new (to_optimized (* rhs))) } Expr :: Opt (expr) => OptimizedExpr :: Opt (Box :: new (to_optimized (* expr))) , Expr :: Rep (expr) => OptimizedExpr :: Rep (Box :: new (to_optimized (* expr))) , Expr :: Skip (strings) => OptimizedExpr :: Skip (strings) , Expr :: Push (expr) => OptimizedExpr :: Push (Box :: new (to_optimized (* expr))) , # [cfg (feature = "grammar-extras")] Expr :: PushLiteral (string) => OptimizedExpr :: PushLiteral (string) , # [cfg (feature = "grammar-extras")] Expr :: NodeTag (expr , tag) => OptimizedExpr :: NodeTag (Box :: new (to_optimized (* expr)) , tag) , # [cfg (feature = "grammar-extras")] Expr :: RepOnce (expr) => OptimizedExpr :: RepOnce (Box :: new (to_optimized (* expr))) , # [cfg (not (feature = "grammar-extras"))] Expr :: RepOnce (_) => unreachable ! ("No valid transformation to OptimizedRule") , Expr :: RepExact (..) | Expr :: RepMin (..) | Expr :: RepMax (..) | Expr :: RepMinMax (..) => { unreachable ! ("No valid transformation to OptimizedRule") } } } OptimizedRule { name : rule . name , ty : rule . ty , expr : to_optimized (rule . expr) , } }
    };
}

rule_to_optimized_rule!();
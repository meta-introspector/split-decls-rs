macro_rules! deps {
    () => {
        Rule!();
        RuleType!();
        Expr!();
    };
}

macro_rules! factor {
    () => {
        deps!();
        pub fn factor (rule : Rule) -> Rule { let Rule { name , ty , expr } = rule ; Rule { name , ty , expr : expr . map_top_down (| expr | { match expr { Expr :: Choice (lhs , rhs) => match (* lhs , * rhs) { (Expr :: Seq (l1 , r1) , Expr :: Seq (l2 , r2)) => { if l1 == l2 { Expr :: Seq (l1 , Box :: new (Expr :: Choice (r1 , r2))) } else { Expr :: Choice (Box :: new (Expr :: Seq (l1 , r1)) , Box :: new (Expr :: Seq (l2 , r2))) } } (Expr :: Seq (l1 , l2) , r) if matches ! (ty , RuleType :: Atomic | RuleType :: CompoundAtomic) => { if * l1 == r { Expr :: Seq (l1 , Box :: new (Expr :: Opt (l2))) } else { Expr :: Choice (Box :: new (Expr :: Seq (l1 , l2)) , Box :: new (r)) } } (l , Expr :: Seq (r1 , r2)) => { if l == * r1 { l } else { Expr :: Choice (Box :: new (l) , Box :: new (Expr :: Seq (r1 , r2))) } } (lhs , rhs) => Expr :: Choice (Box :: new (lhs) , Box :: new (rhs)) , } , expr => expr , } }) , } }
    };
}

factor!();
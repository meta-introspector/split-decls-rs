macro_rules! deps {
    () => {
        Rule!();
        RuleType!();
        Expr!();
    };
}

macro_rules! concatenate {
    () => {
        deps!();
        pub fn concatenate (rule : Rule) -> Rule { let Rule { name , ty , expr } = rule ; Rule { name , ty , expr : expr . map_bottom_up (| expr | { if ty == RuleType :: Atomic { match expr { Expr :: Seq (lhs , rhs) => match (* lhs , * rhs) { (Expr :: Str (lhs) , Expr :: Str (rhs)) => Expr :: Str (lhs + & rhs) , (Expr :: Insens (lhs) , Expr :: Insens (rhs)) => Expr :: Insens (lhs + & rhs) , (lhs , rhs) => Expr :: Seq (Box :: new (lhs) , Box :: new (rhs)) , } , expr => expr , } } else { expr } }) , } }
    };
}

concatenate!();
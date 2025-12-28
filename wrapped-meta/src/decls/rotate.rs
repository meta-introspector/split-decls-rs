macro_rules! deps {
    () => {
        Rule!();
        Expr!();
    };
}

macro_rules! rotate {
    () => {
        deps!();
        pub fn rotate (rule : Rule) -> Rule { fn rotate_internal (expr : Expr) -> Expr { match expr { Expr :: Seq (lhs , rhs) => { let lhs = * lhs ; match lhs { Expr :: Seq (ll , lr) => { rotate_internal (Expr :: Seq (ll , Box :: new (Expr :: Seq (lr , rhs)))) } lhs => Expr :: Seq (Box :: new (lhs) , rhs) , } } Expr :: Choice (lhs , rhs) => { let lhs = * lhs ; match lhs { Expr :: Choice (ll , lr) => { rotate_internal (Expr :: Choice (ll , Box :: new (Expr :: Choice (lr , rhs)))) } lhs => Expr :: Choice (Box :: new (lhs) , rhs) , } } expr => expr , } } let Rule { name , ty , expr } = rule ; Rule { name , ty , expr : expr . map_top_down (rotate_internal) , } }
    };
}

rotate!()
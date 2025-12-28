macro_rules! deps {
    () => {
        Expr!();
        Rule!();
    };
}

macro_rules! list {
    () => {
        deps!();
        pub fn list (rule : Rule) -> Rule { let Rule { name , ty , expr } = rule ; Rule { name , ty , expr : expr . map_bottom_up (| expr | { match expr { Expr :: Seq (l , r) => match * l { Expr :: Rep (l) => { let l = * l ; match l { Expr :: Seq (l1 , l2) => { if l1 == r { Expr :: Seq (l1 , Box :: new (Expr :: Rep (Box :: new (Expr :: Seq (l2 , r))))) } else { Expr :: Seq (Box :: new (Expr :: Rep (Box :: new (Expr :: Seq (l1 , l2)))) , r) } } expr => Expr :: Seq (Box :: new (Expr :: Rep (Box :: new (expr))) , r) , } } expr => Expr :: Seq (Box :: new (expr) , r) , } , expr => expr , } }) , } }
    };
}

list!();
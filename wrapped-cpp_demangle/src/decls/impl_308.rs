macro_rules! deps {
    () => {
        IndexStr!();
        Expression!();
        Result!();
        Parse!();
        ParseContext!();
        FoldExpr!();
        SubstitutionTable!();
        Error!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl Parse for FoldExpr { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (FoldExpr , IndexStr < 'b >) > { try_begin_parse ! ("FoldExpr" , ctx , input) ; let tail = consume (b"f" , input) ? ; if let Ok (tail) = consume (b"l" , tail) { let (operator , tail) = SimpleOperatorName :: parse (ctx , subs , tail) ? ; if operator . arity () != 2 { return Err (error :: Error :: UnexpectedText) ; } let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; return Ok ((FoldExpr :: UnaryLeft (operator , Box :: new (expr)) , tail)) ; } if let Ok (tail) = consume (b"r" , tail) { let (operator , tail) = SimpleOperatorName :: parse (ctx , subs , tail) ? ; if operator . arity () != 2 { return Err (error :: Error :: UnexpectedText) ; } let (expr , tail) = Expression :: parse (ctx , subs , tail) ? ; return Ok ((FoldExpr :: UnaryRight (operator , Box :: new (expr)) , tail)) ; } if let Ok (tail) = consume (b"L" , tail) { let (operator , tail) = SimpleOperatorName :: parse (ctx , subs , tail) ? ; if operator . arity () != 2 { return Err (error :: Error :: UnexpectedText) ; } let (expr1 , tail) = Expression :: parse (ctx , subs , tail) ? ; let (expr2 , tail) = Expression :: parse (ctx , subs , tail) ? ; return Ok ((FoldExpr :: BinaryLeft (operator , Box :: new (expr1) , Box :: new (expr2)) , tail ,)) ; } if let Ok (tail) = consume (b"R" , tail) { let (operator , tail) = SimpleOperatorName :: parse (ctx , subs , tail) ? ; if operator . arity () != 2 { return Err (error :: Error :: UnexpectedText) ; } let (expr1 , tail) = Expression :: parse (ctx , subs , tail) ? ; let (expr2 , tail) = Expression :: parse (ctx , subs , tail) ? ; return Ok ((FoldExpr :: BinaryRight (operator , Box :: new (expr1) , Box :: new (expr2)) , tail ,)) ; } Err (error :: Error :: UnexpectedText) } }
    };
}

impl_308!()
macro_rules! deps {
    () => {
        ParseError!();
        Stmt!();
        AList!();
        StmtList!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , A > TryFrom < Pair < 'a , Rule > > for StmtList < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let inner = p . into_inner () ; let mut stmts = Vec :: new () ; for stmt in inner { stmts . push (Stmt :: try_from (stmt) ?) ; } Ok (StmtList { stmts }) } }
    };
}

impl_25!();
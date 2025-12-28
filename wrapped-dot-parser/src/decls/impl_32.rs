macro_rules! deps {
    () => {
        NodeStmt!();
        Subgraph!();
        Stmt!();
        EdgeStmt!();
        IDEq!();
        ParseError!();
        AList!();
        AttrStmt!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a , A > TryFrom < Pair < 'a , Rule > > for Stmt < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let inner = p . clone () . into_inner () . next () . ok_or (ParseError :: missing_pair (p . clone () , std :: vec ! [Rule :: node_stmt , Rule :: edge_stmt , Rule :: attr_stmt , Rule :: id_eq , Rule :: subgraph ,] ,)) ? ; match inner . as_rule () { Rule :: node_stmt => NodeStmt :: try_from (inner) . map (Stmt :: NodeStmt) , Rule :: edge_stmt => EdgeStmt :: try_from (inner) . map (Stmt :: EdgeStmt) , Rule :: attr_stmt => AttrStmt :: try_from (inner) . map (Stmt :: AttrStmt) , Rule :: id_eq => { let mut inners = inner . into_inner () ; let id1 = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident])) ? . as_str () . into () ; let id2 = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident])) ? . as_str () . into () ; Ok (Stmt :: IDEq (id1 , id2)) } Rule :: subgraph => Subgraph :: try_from (inner) . map (Stmt :: Subgraph) , other => { let error = ParseError :: expect_rule (vec ! [Rule :: node_stmt , Rule :: edge_stmt , Rule :: attr_stmt , Rule :: id_eq , Rule :: subgraph ,] , other ,) ; Err (error) } } } }
    };
}

impl_32!();
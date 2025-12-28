macro_rules! deps {
    () => {
        ParseError!();
        StmtList!();
        Subgraph!();
        AList!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a , A > TryFrom < Pair < 'a , Rule > > for Subgraph < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let mut inner = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident , Rule :: stmt_list] ,)) ? ; let id = if let Rule :: ident = inner . as_rule () { let id_str = inner . as_str () . to_string () ; inner = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: stmt_list])) ? ; Some (id_str) } else { None } ; let stmts = StmtList :: try_from (inner) ? ; Ok (Subgraph { id , stmts }) } }
    };
}

impl_77!()
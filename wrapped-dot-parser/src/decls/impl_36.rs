macro_rules! deps {
    () => {
        Graph!();
        Node!();
        ParseError!();
        AttrStmt!();
        Edge!();
        AttrList!();
        AList!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a , A > TryFrom < Pair < 'a , Rule > > for AttrStmt < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let kind = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: graph , Rule :: node , Rule :: edge] ,)) ? . as_rule () ; let attr_list_pair = inners . next () . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: attr_list])) ? ; let attr = AttrList :: try_from (attr_list_pair) ? ; match kind { Rule :: graph => Ok (AttrStmt :: Graph (attr)) , Rule :: node => Ok (AttrStmt :: Node (attr)) , Rule :: edge => Ok (AttrStmt :: Edge (attr)) , r => Err (ParseError :: expect_rule (vec ! [Rule :: graph , Rule :: node , Rule :: edge] , r ,)) , } } }
    };
}

impl_36!()
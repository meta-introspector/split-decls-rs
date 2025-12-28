macro_rules! deps {
    () => {
        NodeID!();
        EdgeRHS!();
        AList!();
        ParseError!();
        Subgraph!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'a , A > TryFrom < Pair < 'a , Rule > > for EdgeRHS < A > where AList < A > : TryFrom < Pair < 'a , Rule > , Error = ParseError < 'a > > , { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let to_pair = inners . next () . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: node_id , Rule :: subgraph] ,)) ? ; let to = match to_pair . as_rule () { Rule :: node_id => Either :: Left (NodeID :: try_from (to_pair) ?) , Rule :: subgraph => { Either :: Right (Subgraph :: try_from (to_pair) ?) } r => { return Err (ParseError :: expect_rule (vec ! [Rule :: node_id , Rule :: subgraph] , r ,)) ; } } ; let next = inners . next () . map (EdgeRHS :: try_from) . transpose () ? . map (Box :: new) ; Ok (EdgeRHS { to , next }) } }
    };
}

impl_62!()
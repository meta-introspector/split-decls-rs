macro_rules! deps {
    () => {
        Port!();
        NodeID!();
        ParseError!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a > TryFrom < Pair < 'a , Rule > > for NodeID { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let id = inners . next () . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: node_id])) ? . as_str () . to_string () ; let port = inners . next () . map (Port :: try_from) . transpose () ? ; Ok (NodeID { id , port }) } }
    };
}

impl_70!()
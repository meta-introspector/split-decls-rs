macro_rules! deps {
    () => {
        ID!();
        Port!();
        CompassPt!();
        ParseError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a > TryFrom < Pair < 'a , Rule > > for Port { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut inners = p . clone () . into_inner () ; let inner = inners . next () . ok_or (ParseError :: missing_pair (p , vec ! [Rule :: compass_pt , Rule :: ident] ,)) ? ; match inner . as_rule () { Rule :: compass_pt => Ok (Port :: Compass (CompassPt :: try_from (inner) ?)) , Rule :: ident => { let opt_comp = inners . next () . map (CompassPt :: try_from) . transpose () ? ; Ok (Port :: ID (inner . as_str () . to_string () , opt_comp)) } r => Err (ParseError :: expect_rule (vec ! [Rule :: compass_pt , Rule :: ident] , r ,)) , } } }
    };
}

impl_73!();
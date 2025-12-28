macro_rules! deps {
    () => {
        ParseError!();
        ID!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 'a > TryFrom < Pair < 'a , Rule > > for ID < 'a > { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let inner = p . clone () . into_inner () . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident1 , Rule :: numeral , Rule :: quote , Rule :: html] ,)) ? ; let id = match inner . as_rule () { Rule :: ident1 => ID (inner . as_str ()) , Rule :: numeral => ID (inner . as_str ()) , Rule :: quote => { let mut inner = inner . into_inner () ; inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: quotemark])) ? ; let text = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: quote_escaped] ,)) ? ; ID (text . as_str ()) } Rule :: html => ID (inner . as_str ()) , _ => Err (ParseError :: expect_rule (vec ! [Rule :: ident1 , Rule :: numeral , Rule :: quote , Rule :: html] , p . as_rule () ,)) ? , } ; Ok (id) } }
    };
}

impl_84!();
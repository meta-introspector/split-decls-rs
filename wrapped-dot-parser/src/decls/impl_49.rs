macro_rules! deps {
    () => {
        AList!();
        ParseError!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'a > TryFrom < Pair < 'a , Rule > > for AList < (String , String) > { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let mut v : Vec < (String , String) > = Vec :: new () ; let mut inners = p . clone () . into_inner () ; let id1 = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident])) ? . as_str () . into () ; let id2 = inners . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident])) ? . as_str () . into () ; let mut tail = inners . next () . map (| p | { AList :: try_from (p) . map (| alist | alist . elems) . unwrap_or_default () }) . unwrap_or_default () ; v . push ((id1 , id2)) ; v . append (& mut tail) ; Ok (AList { elems : v }) } }
    };
}

impl_49!()
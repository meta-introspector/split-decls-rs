macro_rules! deps {
    () => {
        RealRegexMatcher!();
        RealRegexCaptures!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl RegexMatcher for RealRegexMatcher { fn new (re : & str) -> Result < Self , String > { let regex = Regex :: new (re) . map_err (| e | e . to_string ()) ? ; Ok (RealRegexMatcher { regex }) } fn is_match (& self , text : & str) -> bool { self . regex . is_match (text) } fn captures < 't > (& 't self , text : & 't str) -> Option < Box < dyn RegexCaptures + 't > > { self . regex . captures (text) . map (| caps | Box :: new (RealRegexCaptures { captures : caps }) as Box < dyn RegexCaptures >) } }
    };
}

impl_1!()
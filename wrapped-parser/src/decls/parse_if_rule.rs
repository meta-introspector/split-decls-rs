macro_rules! deps {
    () => {
        Rule!();
        Result!();
    };
}

macro_rules! parse_if_rule {
    () => {
        deps!();
        pub (super) fn parse_if_rule < T > (pairs : & mut Pairs < Rule > , rule : Rule , f : impl FnOnce (Pair < Rule >) -> Result < T > ,) -> Result < Option < T > > { next_if_rule (pairs , rule) . map (f) . transpose () }
    };
}

parse_if_rule!();
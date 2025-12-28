macro_rules! deps {
    () => {
        Selection!();
        FragmentSpread!();
        Rule!();
        Positioned!();
        Result!();
        PositionCalculator!();
        Field!();
        InlineFragment!();
    };
}

macro_rules! parse_selection {
    () => {
        deps!();
        fn parse_selection (pair : Pair < Rule > , pc : & mut PositionCalculator , remaining_depth : usize ,) -> Result < Positioned < Selection > > { debug_assert_eq ! (pair . as_rule () , Rule :: selection) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; Ok (Positioned :: new (match pair . as_rule () { Rule :: field => Selection :: Field (parse_field (pair , pc , remaining_depth) ?) , Rule :: fragment_spread => Selection :: FragmentSpread (parse_fragment_spread (pair , pc) ?) , Rule :: inline_fragment => { Selection :: InlineFragment (parse_inline_fragment (pair , pc , remaining_depth) ?) } _ => unreachable ! () , } , pos ,)) }
    };
}

parse_selection!();
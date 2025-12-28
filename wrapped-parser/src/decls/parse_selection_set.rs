macro_rules! deps {
    () => {
        Result!();
        SelectionSet!();
        PositionCalculator!();
        Rule!();
        Positioned!();
    };
}

macro_rules! parse_selection_set {
    () => {
        deps!();
        fn parse_selection_set (pair : Pair < Rule > , pc : & mut PositionCalculator , remaining_depth : usize ,) -> Result < Positioned < SelectionSet > > { debug_assert_eq ! (pair . as_rule () , Rule :: selection_set) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (SelectionSet { items : pair . into_inner () . map (| pair | parse_selection (pair , pc , remaining_depth)) . collect :: < Result < _ > > () ? , } , pos ,)) }
    };
}

parse_selection_set!()
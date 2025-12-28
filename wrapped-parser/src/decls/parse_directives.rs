macro_rules! deps {
    () => {
        Rule!();
        PositionCalculator!();
        Result!();
        Positioned!();
        Directive!();
    };
}

macro_rules! parse_directives {
    () => {
        deps!();
        fn parse_directives (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < Directive > > > { debug_assert_eq ! (pair . as_rule () , Rule :: directives) ; pair . into_inner () . map (| pair | parse_directive (pair , pc)) . collect () }
    };
}

parse_directives!();
macro_rules! deps {
    () => {
        Result!();
        Rule!();
        Positioned!();
        ConstDirective!();
        PositionCalculator!();
    };
}

macro_rules! parse_const_directives {
    () => {
        deps!();
        fn parse_const_directives (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < ConstDirective > > > { debug_assert_eq ! (pair . as_rule () , Rule :: const_directives) ; pair . into_inner () . map (| pair | parse_const_directive (pair , pc)) . collect () }
    };
}

parse_const_directives!();
macro_rules! deps {
    () => {
        Positioned!();
        PositionCalculator!();
        Result!();
        ConstDirective!();
        Rule!();
    };
}

macro_rules! parse_opt_const_directives {
    () => {
        deps!();
        fn parse_opt_const_directives (pairs : & mut Pairs < '_ , Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < ConstDirective > > > { Ok (parse_if_rule (pairs , Rule :: const_directives , | pair | { parse_const_directives (pair , pc) }) ? . unwrap_or_default ()) }
    };
}

parse_opt_const_directives!()
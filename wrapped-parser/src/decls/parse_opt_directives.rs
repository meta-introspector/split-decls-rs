macro_rules! deps {
    () => {
        Result!();
        Rule!();
        Directive!();
        Positioned!();
        PositionCalculator!();
    };
}

macro_rules! parse_opt_directives {
    () => {
        deps!();
        fn parse_opt_directives (pairs : & mut Pairs < '_ , Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < Directive > > > { Ok (parse_if_rule (pairs , Rule :: directives , | pair | parse_directives (pair , pc)) ? . unwrap_or_default () ,) }
    };
}

parse_opt_directives!()
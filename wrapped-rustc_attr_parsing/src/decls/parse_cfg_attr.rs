macro_rules! deps {
    () => {
        ArgParser!();
        AcceptContext!();
        Stage!();
    };
}

macro_rules! parse_cfg_attr {
    () => {
        deps!();
        pub fn parse_cfg_attr < 'c , S : Stage > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> Option < CfgEntry > { let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; parse_cfg_entry (cx , single) }
    };
}

parse_cfg_attr!()
macro_rules! deps {
    () => {
        Error!();
        Positioned!();
        Result!();
        Rule!();
        PositionCalculator!();
    };
}

macro_rules! parse_number {
    () => {
        deps!();
        fn parse_number (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Number > > { debug_assert_eq ! (pair . as_rule () , Rule :: number) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (pair . as_str () . parse () . map_err (| err | Error :: Syntax { message : format ! ("invalid number: {}" , err) , start : pos , end : None , }) ? , pos ,)) }
    };
}

parse_number!();
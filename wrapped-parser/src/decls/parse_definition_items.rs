macro_rules! deps {
    () => {
        DefinitionItem!();
        Result!();
        Rule!();
        PositionCalculator!();
    };
}

macro_rules! parse_definition_items {
    () => {
        deps!();
        fn parse_definition_items (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < DefinitionItem > > { debug_assert_eq ! (pair . as_rule () , Rule :: executable_document) ; Ok (pair . into_inner () . filter (| pair | pair . as_rule () != Rule :: EOI) . map (| pair | parse_definition_item (pair , pc)) . collect :: < Result < _ > > () ?) }
    };
}

parse_definition_items!()
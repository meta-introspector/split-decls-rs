macro_rules! deps {
    () => {
        Rule!();
        Result!();
        ServiceDocument!();
        PositionCalculator!();
    };
}

macro_rules! parse_service_document {
    () => {
        deps!();
        fn parse_service_document (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < ServiceDocument > { debug_assert_eq ! (pair . as_rule () , Rule :: service_document) ; Ok (ServiceDocument { definitions : pair . into_inner () . filter (| pair | pair . as_rule () != Rule :: EOI) . map (| pair | parse_type_system_definition (pair , pc)) . collect :: < Result < _ > > () ? , }) }
    };
}

parse_service_document!()
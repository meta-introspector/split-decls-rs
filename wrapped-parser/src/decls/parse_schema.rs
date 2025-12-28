macro_rules! deps {
    () => {
        ServiceDocument!();
        GraphQLParser!();
        Result!();
        PositionCalculator!();
        Rule!();
    };
}

macro_rules! parse_schema {
    () => {
        deps!();
        # [doc = " Parse a GraphQL schema document."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if the schema is not a valid GraphQL document."] pub fn parse_schema < T : AsRef < str > > (input : T) -> Result < ServiceDocument > { let mut pc = PositionCalculator :: new (input . as_ref ()) ; Ok (parse_service_document (exactly_one (GraphQLParser :: parse (Rule :: service_document , input . as_ref () ,) ?) , & mut pc ,) ?) }
    };
}

parse_schema!();
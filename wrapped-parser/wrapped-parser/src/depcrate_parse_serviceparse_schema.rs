// Generated macro for parse_schema (function)
macro_rules! Depcrate_parse_serviceparse_schema {
() => {
// Module: crate::parse::service
// Provides: {"parse_schema"}
// Dependencies: {}
# [doc = " Parse a GraphQL schema document."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if the schema is not a valid GraphQL document."] pub fn parse_schema < T : AsRef < str > > (input : T) -> Result < ServiceDocument > { let mut pc = PositionCalculator :: new (input . as_ref ()) ; Ok (parse_service_document (exactly_one (GraphQLParser :: parse (Rule :: service_document , input . as_ref () ,) ?) , & mut pc ,) ?) }
};
}

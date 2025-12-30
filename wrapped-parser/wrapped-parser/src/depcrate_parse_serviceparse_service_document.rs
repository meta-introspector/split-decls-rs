// Generated macro for parse_service_document (function)
macro_rules! Depcrate_parse_serviceparse_service_document {
() => {
// Module: crate::parse::service
// Provides: {"parse_service_document"}
// Dependencies: {}
fn parse_service_document (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < ServiceDocument > { debug_assert_eq ! (pair . as_rule () , Rule :: service_document) ; Ok (ServiceDocument { definitions : pair . into_inner () . filter (| pair | pair . as_rule () != Rule :: EOI) . map (| pair | parse_type_system_definition (pair , pc)) . collect :: < Result < _ > > () ? , }) }
};
}

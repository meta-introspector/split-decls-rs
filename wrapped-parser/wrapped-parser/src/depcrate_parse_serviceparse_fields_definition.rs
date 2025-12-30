// Generated macro for parse_fields_definition (function)
macro_rules! Depcrate_parse_serviceparse_fields_definition {
() => {
// Module: crate::parse::service
// Provides: {"parse_fields_definition"}
// Dependencies: {}
fn parse_fields_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < FieldDefinition > > > { debug_assert_eq ! (pair . as_rule () , Rule :: fields_definition) ; pair . into_inner () . map (| pair | parse_field_definition (pair , pc)) . collect () }
};
}

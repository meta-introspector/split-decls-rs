// Generated macro for array (function)
macro_rules! Depcrate_jsonarray {
() => {
// Module: crate::json
// Provides: {"array"}
// Dependencies: {}
fn array (input : & str) -> IResult < & str , Vec < JsonValue > > { delimited (char ('[') , ws (separated_list0 (ws (char (',')) , json_value)) , char (']') ,) . parse (input) }
};
}

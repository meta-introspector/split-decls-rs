// Generated macro for object (function)
macro_rules! Depcrate_jsonobject {
() => {
// Module: crate::json
// Provides: {"object"}
// Dependencies: {}
fn object (input : & str) -> IResult < & str , HashMap < String , JsonValue > > { map (delimited (char ('{') , ws (separated_list0 (ws (char (',')) , separated_pair (string , ws (char (':')) , json_value) ,)) , char ('}') ,) , | key_values | key_values . into_iter () . collect () ,) . parse (input) }
};
}

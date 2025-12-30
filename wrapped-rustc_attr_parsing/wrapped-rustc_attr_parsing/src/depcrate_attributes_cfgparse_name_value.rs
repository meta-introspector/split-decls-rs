// Generated macro for parse_name_value (function)
macro_rules! Depcrate_attributes_cfgparse_name_value {
() => {
// Module: crate::attributes::cfg
// Provides: {"parse_name_value"}
// Dependencies: {}
fn parse_name_value < S : Stage > (name : Symbol , name_span : Span , value : Option < & NameValueParser > , span : Span , cx : & mut AcceptContext < '_ , '_ , S > ,) -> Option < CfgEntry > { try_gate_cfg (name , span , cx . sess () , cx . features_option ()) ; let value = match value { None => None , Some (value) => { let Some (value_str) = value . value_as_str () else { cx . expected_string_literal (value . value_span , Some (value . value_as_lit ())) ; return None ; } ; Some ((value_str , value . value_span)) } } ; Some (CfgEntry :: NameValue { name , name_span , value , span }) }
};
}

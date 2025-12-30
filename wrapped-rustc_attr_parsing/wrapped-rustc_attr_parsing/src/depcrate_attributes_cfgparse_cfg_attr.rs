// Generated macro for parse_cfg_attr (function)
macro_rules! Depcrate_attributes_cfgparse_cfg_attr {
() => {
// Module: crate::attributes::cfg
// Provides: {"parse_cfg_attr"}
// Dependencies: {}
pub fn parse_cfg_attr < 'c , S : Stage > (cx : & 'c mut AcceptContext < '_ , '_ , S > , args : & 'c ArgParser < '_ > ,) -> Option < CfgEntry > { let ArgParser :: List (list) = args else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; parse_cfg_entry (cx , single) }
};
}

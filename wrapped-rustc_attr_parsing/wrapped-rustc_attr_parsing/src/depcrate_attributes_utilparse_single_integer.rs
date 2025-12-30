// Generated macro for parse_single_integer (function)
macro_rules! Depcrate_attributes_utilparse_single_integer {
() => {
// Module: crate::attributes::util
// Provides: {"parse_single_integer"}
// Dependencies: {}
# [doc = " Parse a single integer."] # [doc = ""] # [doc = " Used by attributes that take a single integer as argument, such as"] # [doc = " `#[link_ordinal]` and `#[rustc_layout_scalar_valid_range_start]`."] # [doc = " `cx` is the context given to the attribute."] # [doc = " `args` is the parser for the attribute arguments."] pub (crate) fn parse_single_integer < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ > ,) -> Option < u128 > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; let Some (lit) = single . lit () else { cx . expected_integer_literal (single . span ()) ; return None ; } ; let LitKind :: Int (num , _ty) = lit . kind else { cx . expected_integer_literal (single . span ()) ; return None ; } ; Some (num . 0) }
};
}

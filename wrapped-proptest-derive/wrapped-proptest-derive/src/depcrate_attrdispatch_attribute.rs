// Generated macro for dispatch_attribute (function)
macro_rules! Depcrate_attrdispatch_attribute {
() => {
// Module: crate::attr
// Provides: {"dispatch_attribute"}
// Dependencies: {}
# [doc = " Dispatches an attribute modifier to handlers and"] # [doc = " let's them add stuff into our accumulartor."] fn dispatch_attribute (ctx : Ctx , mut acc : ParseAcc , meta : Meta) -> ParseAcc { let path = meta . path () ; if let Some (name) = path . get_ident () . map (ToString :: to_string) { match name . as_ref () { "skip" => parse_skip (ctx , & mut acc , meta) , "w" | "weight" => parse_weight (ctx , & mut acc , & meta) , "no_params" => parse_no_params (ctx , & mut acc , meta) , "params" => parse_params (ctx , & mut acc , meta) , "strategy" => parse_strategy (ctx , & mut acc , & meta) , "value" => parse_value (ctx , & mut acc , & meta) , "regex" => parse_regex (ctx , & mut acc , & meta) , "filter" => parse_filter (ctx , & mut acc , & meta) , "no_bound" => parse_no_bound (ctx , & mut acc , meta) , name => dispatch_unknown_mod (ctx , name) , } } else { error :: unkown_modifier (ctx , & path . into_token_stream () . to_string ()) ; } acc }
};
}

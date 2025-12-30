// Generated macro for parse_params (function)
macro_rules! Depcrate_attrparse_params {
() => {
// Module: crate::attr
// Provides: {"parse_params"}
// Dependencies: {}
# [doc = " Parses an explicit Parameters type."] # [doc = ""] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(params(<type>)]`"] # [doc = " + `#[proptest(params(\"<type>\")]`"] # [doc = " + `#[proptest(params = \"<type>\"]`"] # [doc = ""] # [doc = " The latter form is required for more complex types."] fn parse_params (ctx : Ctx , acc : & mut ParseAcc , meta : Meta) { error_if_set (ctx , & acc . params , & meta) ; let typ = match normalize_meta (meta) { Some (NormMeta :: Word (ident)) => Some (ident_to_type (ident)) , Some (NormMeta :: Lit (Lit :: Str (lit))) => lit . parse () . ok () , _ => None , } ; if let typ @ Some (_) = typ { acc . params = typ ; } else { error :: param_malformed (ctx) } }
};
}

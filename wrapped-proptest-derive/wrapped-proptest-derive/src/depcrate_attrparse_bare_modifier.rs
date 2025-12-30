// Generated macro for parse_bare_modifier (function)
macro_rules! Depcrate_attrparse_bare_modifier {
() => {
// Module: crate::attr
// Provides: {"parse_bare_modifier"}
// Dependencies: {}
# [doc = " Parses a bare attribute of the form `#[proptest(<attr>)]` and sets `loc`."] fn parse_bare_modifier (ctx : Ctx , loc : & mut Option < () > , meta : Meta , malformed : fn (Ctx) ,) { error_if_set (ctx , loc , & meta) ; if let Some (NormMeta :: Plain) = normalize_meta (meta) { * loc = Some (()) ; } else { malformed (ctx) ; } }
};
}

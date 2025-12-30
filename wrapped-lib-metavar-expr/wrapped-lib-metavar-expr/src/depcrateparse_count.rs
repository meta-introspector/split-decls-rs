// Generated macro for parse_count (function)
macro_rules! Depcrateparse_count {
() => {
// Module: crate
// Provides: {"parse_count"}
// Dependencies: {}
# [doc = " Parse a meta-variable `count` expression: `count(ident[, depth])`"] fn parse_count < 'psess > (iter : & mut TokenStreamIter < '_ > , psess : & 'psess ParseSess , span : Span ,) -> PResult < 'psess , MetaVarExpr > { eat_dollar (iter , psess , span) ? ; let ident = parse_ident (iter , psess , span) ? ; let depth = if try_eat_comma (iter) { if iter . peek () . is_none () { return Err (psess . dcx () . struct_span_err (span , "`count` followed by a comma must have an associated index indicating its depth" ,)) ; } parse_depth (iter , psess , span) ? } else { 0 } ; Ok (MetaVarExpr :: Count (ident , depth)) }
};
}

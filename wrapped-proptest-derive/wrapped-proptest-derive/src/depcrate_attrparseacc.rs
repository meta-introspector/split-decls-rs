// Generated macro for ParseAcc (struct)
macro_rules! Depcrate_attrParseAcc {
() => {
// Module: crate::attr
// Provides: {"ParseAcc"}
// Dependencies: {}
# [doc = " The internal state of the attribute parser."] # [derive (Default)] struct ParseAcc { skip : Option < () > , weight : Option < u32 > , no_params : Option < () > , params : Option < Type > , strategy : Option < Expr > , value : Option < Expr > , regex : Option < Expr > , filter : Vec < Expr > , no_bound : Option < () > , }
};
}

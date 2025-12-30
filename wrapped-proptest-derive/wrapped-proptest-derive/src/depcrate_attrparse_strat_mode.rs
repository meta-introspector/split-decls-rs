// Generated macro for parse_strat_mode (function)
macro_rules! Depcrate_attrparse_strat_mode {
() => {
// Module: crate::attr
// Provides: {"parse_strat_mode"}
// Dependencies: {}
# [doc = " Combines any parsed explicit strategy, value, and regex into a single"] # [doc = " value and fails if both an explicit strategy / value / regex was set."] # [doc = " Only one of them can be set, or none."] fn parse_strat_mode (ctx : Ctx , strat : Option < Expr > , value : Option < Expr > , regex : Option < Expr > ,) -> DeriveResult < StratMode > { Ok (match (strat , value , regex) { (None , None , None) => StratMode :: Arbitrary , (None , None , Some (re)) => StratMode :: Regex (re) , (None , Some (vl) , None) => StratMode :: Value (vl) , (Some (st) , None , None) => StratMode :: Strategy (st) , _ => error :: overspecified_strat (ctx) ? , }) }
};
}

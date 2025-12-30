// Generated macro for impl_120 (impl)
macro_rules! Depcrateimpl_120 {
() => {
// Module: crate
// Provides: {"impl_120"}
// Dependencies: {}
impl std :: fmt :: Display for RelationalOperator { fn fmt (& self , fmt : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { match * self { RelationalOperator :: LessOrEqual => write ! (fmt , "<=") ? , RelationalOperator :: Equal => write ! (fmt , "==") ? , RelationalOperator :: GreaterOrEqual => write ! (fmt , ">=") ? , } ; Ok (()) } }
};
}

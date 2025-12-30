// Generated macro for tests (module)
macro_rules! Depcrate_valuetests {
() => {
// Module: crate::value
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ValueKind ; use crate :: Config ; use crate :: File ; use crate :: FileFormat ; # [test] # [cfg (feature = "toml")] fn test_i64 () { let c = Config :: builder () . add_source (File :: from_str ("
value = 120
" , FileFormat :: Toml ,)) . build () . unwrap () ; assert ! (std :: matches ! (c . cache . kind , ValueKind :: Table (_))) ; let v = match c . cache . kind { ValueKind :: Table (t) => t , _ => unreachable ! () , } ; let value = v . get ("value") . unwrap () ; assert ! (std :: matches ! (value . kind , ValueKind :: I64 (120)) , "Is not a i64(120): {:?}" , value . kind) ; } }
};
}

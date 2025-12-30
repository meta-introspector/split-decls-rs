// Generated macro for impl_11 (impl)
macro_rules! Depcrate_optimpl_11 {
() => {
// Module: crate::opt
// Provides: {"impl_11"}
// Dependencies: {}
impl std :: str :: FromStr for Optimization { type Err = InvalidOptimizationLevel ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "0" | "O0" => Ok (Optimization :: O0) , "1" | "O1" => Ok (Optimization :: O1) , "2" | "O2" => Ok (Optimization :: O2) , "3" | "O3" => Ok (Optimization :: O3) , "s" | "Os" => Ok (Optimization :: Os) , "z" | "Oz" => Ok (Optimization :: Oz) , _ => Err (InvalidOptimizationLevel) , } } }
};
}

// Generated macro for impl_56 (impl)
macro_rules! Depcrate_outputimpl_56 {
() => {
// Module: crate::output
// Provides: {"impl_56"}
// Dependencies: {}
impl :: std :: str :: FromStr for Algorithm { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s . to_lowercase () . as_ref () { "naive" => Ok (Algorithm :: Naive) , "datafrogopt" => Ok (Algorithm :: DatafrogOpt) , "locationinsensitive" => Ok (Algorithm :: LocationInsensitive) , "compare" => Ok (Algorithm :: Compare) , "hybrid" => Ok (Algorithm :: Hybrid) , _ => Err (String :: from ("valid values: Naive, DatafrogOpt, LocationInsensitive, Compare, Hybrid" ,)) , } } }
};
}

// Generated macro for impl_8 (impl)
macro_rules! Depcrate_lineimpl_8 {
() => {
// Module: crate::line
// Provides: {"impl_8"}
// Dependencies: {}
impl FromStr for Year { type Err = Error ; fn from_str (input : & str) -> Result < Year , Self :: Err > { Ok (match & * input . to_ascii_lowercase () { "min" | "minimum" => Year :: Minimum , "max" | "maximum" => Year :: Maximum , year => match year . parse () { Ok (year) => Year :: Number (year) , Err (_) => return Err (Error :: FailedYearParse (input . to_string ())) , } , }) } }
};
}

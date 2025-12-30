// Generated macro for impl_11 (impl)
macro_rules! Depcrate_lineimpl_11 {
() => {
// Module: crate::line
// Provides: {"impl_11"}
// Dependencies: {}
impl FromStr for Month { type Err = Error ; # [doc = " Attempts to parse the given string into a value of this type."] fn from_str (input : & str) -> Result < Month , Self :: Err > { Ok (match & * input . to_ascii_lowercase () { "jan" | "january" => Month :: January , "feb" | "february" => Month :: February , "mar" | "march" => Month :: March , "apr" | "april" => Month :: April , "may" => Month :: May , "jun" | "june" => Month :: June , "jul" | "july" => Month :: July , "aug" | "august" => Month :: August , "sep" | "september" => Month :: September , "oct" | "october" => Month :: October , "nov" | "november" => Month :: November , "dec" | "december" => Month :: December , other => return Err (Error :: FailedMonthParse (other . to_string ())) , }) } }
};
}

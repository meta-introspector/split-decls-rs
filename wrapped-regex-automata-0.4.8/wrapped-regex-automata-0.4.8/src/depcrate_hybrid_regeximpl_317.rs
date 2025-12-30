// Generated macro for impl_317 (impl)
macro_rules! Depcrate_hybrid_regeximpl_317 {
() => {
// Module: crate::hybrid::regex
// Provides: {"impl_317"}
// Dependencies: {}
# [doc = " Non-search APIs for querying information about the regex and setting a"] # [doc = " prefilter."] impl Regex { # [doc = " Return the underlying lazy DFA responsible for forward matching."] # [doc = ""] # [doc = " This is useful for accessing the underlying lazy DFA and using it"] # [doc = " directly if the situation calls for it."] pub fn forward (& self) -> & DFA { & self . forward } # [doc = " Return the underlying lazy DFA responsible for reverse matching."] # [doc = ""] # [doc = " This is useful for accessing the underlying lazy DFA and using it"] # [doc = " directly if the situation calls for it."] pub fn reverse (& self) -> & DFA { & self . reverse } # [doc = " Returns the total number of patterns matched by this regex."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return Ok(()); } // miri takes too long"] # [doc = " use regex_automata::hybrid::regex::Regex;"] # [doc = ""] # [doc = " let re = Regex::new_many(&[r\"[a-z]+\", r\"[0-9]+\", r\"\\w+\"])?;"] # [doc = " assert_eq!(3, re.pattern_len());"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn pattern_len (& self) -> usize { assert_eq ! (self . forward () . pattern_len () , self . reverse () . pattern_len ()) ; self . forward () . pattern_len () } }
};
}

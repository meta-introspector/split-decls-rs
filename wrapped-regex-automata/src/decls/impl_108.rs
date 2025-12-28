macro_rules! deps {
    () => {
        DFA!();
        Automaton!();
        Regex!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [doc = " Non-search APIs for querying information about the regex and setting a"] # [doc = " prefilter."] impl < A : Automaton > Regex < A > { # [doc = " Return the underlying DFA responsible for forward matching."] # [doc = ""] # [doc = " This is useful for accessing the underlying DFA and converting it to"] # [doc = " some other format or size. See the [`Builder::build_from_dfas`] docs"] # [doc = " for an example of where this might be useful."] pub fn forward (& self) -> & A { & self . forward } # [doc = " Return the underlying DFA responsible for reverse matching."] # [doc = ""] # [doc = " This is useful for accessing the underlying DFA and converting it to"] # [doc = " some other format or size. See the [`Builder::build_from_dfas`] docs"] # [doc = " for an example of where this might be useful."] pub fn reverse (& self) -> & A { & self . reverse } # [doc = " Returns the total number of patterns matched by this regex."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return Ok(()); } // miri takes too long"] # [doc = " use regex_automata::dfa::regex::Regex;"] # [doc = ""] # [doc = " let re = Regex::new_many(&[r\"[a-z]+\", r\"[0-9]+\", r\"\\w+\"])?;"] # [doc = " assert_eq!(3, re.pattern_len());"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn pattern_len (& self) -> usize { assert_eq ! (self . forward () . pattern_len () , self . reverse () . pattern_len ()) ; self . forward () . pattern_len () } }
    };
}

impl_108!();
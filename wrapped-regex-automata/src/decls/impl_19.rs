macro_rules! deps {
    () => {
        ByteClasses!();
        StartTable!();
        Special!();
        StartKind!();
        BuildError!();
        Accels!();
        ByteSet!();
        OwnedDFA!();
        NFA!();
        Flags!();
        DFA!();
        LookMatcher!();
        Builder!();
        Prefilter!();
        MatchStates!();
        HalfMatch!();
        Input!();
        TransitionTable!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl OwnedDFA { # [doc = " Create a new DFA that matches every input."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{dfa::{Automaton, dense}, HalfMatch, Input};"] # [doc = ""] # [doc = " let dfa = dense::DFA::always_match()?;"] # [doc = ""] # [doc = " let expected = Some(HalfMatch::must(0, 0));"] # [doc = " assert_eq!(expected, dfa.try_search_fwd(&Input::new(\"\"))?);"] # [doc = " assert_eq!(expected, dfa.try_search_fwd(&Input::new(\"foo\"))?);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn always_match () -> Result < OwnedDFA , BuildError > { let nfa = thompson :: NFA :: always_match () ; Builder :: new () . build_from_nfa (& nfa) } # [doc = " Create a new DFA that never matches any input."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{dfa::{Automaton, dense}, Input};"] # [doc = ""] # [doc = " let dfa = dense::DFA::never_match()?;"] # [doc = " assert_eq!(None, dfa.try_search_fwd(&Input::new(\"\"))?);"] # [doc = " assert_eq!(None, dfa.try_search_fwd(&Input::new(\"foo\"))?);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn never_match () -> Result < OwnedDFA , BuildError > { let nfa = thompson :: NFA :: never_match () ; Builder :: new () . build_from_nfa (& nfa) } # [doc = " Create an initial DFA with the given equivalence classes, pattern"] # [doc = " length and whether anchored starting states are enabled for each"] # [doc = " pattern. An initial DFA can be further mutated via determinization."] fn initial (classes : ByteClasses , pattern_len : usize , starts : StartKind , lookm : & LookMatcher , starts_for_each_pattern : bool , pre : Option < Prefilter > , quitset : ByteSet , flags : Flags ,) -> Result < OwnedDFA , BuildError > { let start_pattern_len = if starts_for_each_pattern { Some (pattern_len) } else { None } ; Ok (DFA { tt : TransitionTable :: minimal (classes) , st : StartTable :: dead (starts , lookm , start_pattern_len) ? , ms : MatchStates :: empty (pattern_len) , special : Special :: new () , accels : Accels :: empty () , pre , quitset , flags , }) } }
    };
}

impl_19!();
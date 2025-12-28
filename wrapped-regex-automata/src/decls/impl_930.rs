macro_rules! deps {
    () => {
        PatternID!();
        Anchored!();
    };
}

macro_rules! impl_930 {
    () => {
        deps!();
        impl Anchored { # [doc = " Returns true if and only if this anchor mode corresponds to any kind of"] # [doc = " anchored search."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This examples shows that both `Anchored::Yes` and `Anchored::Pattern`"] # [doc = " are considered anchored searches."] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{Anchored, PatternID};"] # [doc = ""] # [doc = " assert!(!Anchored::No.is_anchored());"] # [doc = " assert!(Anchored::Yes.is_anchored());"] # [doc = " assert!(Anchored::Pattern(PatternID::ZERO).is_anchored());"] # [doc = " ```"] # [inline] pub fn is_anchored (& self) -> bool { matches ! (* self , Anchored :: Yes | Anchored :: Pattern (_)) } # [doc = " Returns the pattern ID associated with this configuration if it is an"] # [doc = " anchored search for a specific pattern. Otherwise `None` is returned."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{Anchored, PatternID};"] # [doc = ""] # [doc = " assert_eq!(None, Anchored::No.pattern());"] # [doc = " assert_eq!(None, Anchored::Yes.pattern());"] # [doc = ""] # [doc = " let pid = PatternID::must(5);"] # [doc = " assert_eq!(Some(pid), Anchored::Pattern(pid).pattern());"] # [doc = " ```"] # [inline] pub fn pattern (& self) -> Option < PatternID > { match * self { Anchored :: Pattern (pid) => Some (pid) , _ => None , } } }
    };
}

impl_930!();
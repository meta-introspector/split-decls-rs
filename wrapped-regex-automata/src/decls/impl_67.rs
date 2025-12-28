macro_rules! deps {
    () => {
        InternalBuilder!();
        DFA!();
        Compiler!();
        Config!();
        Builder!();
        NFA!();
        Match!();
        BuildError!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Builder { # [doc = " Create a new one-pass DFA builder with the default configuration."] pub fn new () -> Builder { Builder { config : Config :: default () , # [cfg (feature = "syntax")] thompson : thompson :: Compiler :: new () , } } # [doc = " Build a one-pass DFA from the given pattern."] # [doc = ""] # [doc = " If there was a problem parsing or compiling the pattern, then an error"] # [doc = " is returned."] # [cfg (feature = "syntax")] pub fn build (& self , pattern : & str) -> Result < DFA , BuildError > { self . build_many (& [pattern]) } # [doc = " Build a one-pass DFA from the given patterns."] # [doc = ""] # [doc = " When matches are returned, the pattern ID corresponds to the index of"] # [doc = " the pattern in the slice given."] # [cfg (feature = "syntax")] pub fn build_many < P : AsRef < str > > (& self , patterns : & [P] ,) -> Result < DFA , BuildError > { let nfa = self . thompson . build_many (patterns) . map_err (BuildError :: nfa) ? ; self . build_from_nfa (nfa) } # [doc = " Build a DFA from the given NFA."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to build a DFA if you already have an NFA in"] # [doc = " hand."] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{dfa::onepass::DFA, nfa::thompson::NFA, Match};"] # [doc = ""] # [doc = " // This shows how to set non-default options for building an NFA."] # [doc = " let nfa = NFA::compiler()"] # [doc = "     .configure(NFA::config().shrink(true))"] # [doc = "     .build(r\"[a-z0-9]+\")?;"] # [doc = " let re = DFA::builder().build_from_nfa(nfa)?;"] # [doc = " let (mut cache, mut caps) = (re.create_cache(), re.create_captures());"] # [doc = " re.captures(&mut cache, \"foo123bar\", &mut caps);"] # [doc = " assert_eq!(Some(Match::must(0, 0..9)), caps.get_match());"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn build_from_nfa (& self , nfa : NFA) -> Result < DFA , BuildError > { InternalBuilder :: new (self . config . clone () , & nfa) . build () } # [doc = " Apply the given one-pass DFA configuration options to this builder."] pub fn configure (& mut self , config : Config) -> & mut Builder { self . config = self . config . overwrite (config) ; self } # [doc = " Set the syntax configuration for this builder using"] # [doc = " [`syntax::Config`](crate::util::syntax::Config)."] # [doc = ""] # [doc = " This permits setting things like case insensitivity, Unicode and multi"] # [doc = " line mode."] # [doc = ""] # [doc = " These settings only apply when constructing a one-pass DFA directly"] # [doc = " from a pattern."] # [cfg (feature = "syntax")] pub fn syntax (& mut self , config : crate :: util :: syntax :: Config ,) -> & mut Builder { self . thompson . syntax (config) ; self } # [doc = " Set the Thompson NFA configuration for this builder using"] # [doc = " [`nfa::thompson::Config`](crate::nfa::thompson::Config)."] # [doc = ""] # [doc = " This permits setting things like whether additional time should be"] # [doc = " spent shrinking the size of the NFA."] # [doc = ""] # [doc = " These settings only apply when constructing a DFA directly from a"] # [doc = " pattern."] # [cfg (feature = "syntax")] pub fn thompson (& mut self , config : thompson :: Config) -> & mut Builder { self . thompson . configure (config) ; self } }
    };
}

impl_67!()
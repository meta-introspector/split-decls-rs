macro_rules! deps {
    () => {
        Regex!();
        Builder!();
        BuildError!();
        DFA!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        # [cfg (all (feature = "syntax" , feature = "dfa-build"))] impl Regex < sparse :: DFA < Vec < u8 > > > { # [doc = " Parse the given regular expression using the default configuration,"] # [doc = " except using sparse DFAs, and return the corresponding regex."] # [doc = ""] # [doc = " If you want a non-default configuration, then use the [`Builder`] to"] # [doc = " set your own configuration."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{Match, dfa::regex::Regex};"] # [doc = ""] # [doc = " let re = Regex::new_sparse(\"foo[0-9]+bar\")?;"] # [doc = " assert_eq!("] # [doc = "     Some(Match::must(0, 3..14)),"] # [doc = "     re.find(b\"zzzfoo12345barzzz\"),"] # [doc = " );"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn new_sparse (pattern : & str ,) -> Result < Regex < sparse :: DFA < Vec < u8 > > > , BuildError > { Builder :: new () . build_sparse (pattern) } # [doc = " Like `new`, but parses multiple patterns into a single \"regex set\""] # [doc = " using sparse DFAs. This otherwise similarly uses the default regex"] # [doc = " configuration."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{Match, dfa::regex::Regex};"] # [doc = ""] # [doc = " let re = Regex::new_many_sparse(&[\"[a-z]+\", \"[0-9]+\"])?;"] # [doc = ""] # [doc = " let mut it = re.find_iter(b\"abc 1 foo 4567 0 quux\");"] # [doc = " assert_eq!(Some(Match::must(0, 0..3)), it.next());"] # [doc = " assert_eq!(Some(Match::must(1, 4..5)), it.next());"] # [doc = " assert_eq!(Some(Match::must(0, 6..9)), it.next());"] # [doc = " assert_eq!(Some(Match::must(1, 10..14)), it.next());"] # [doc = " assert_eq!(Some(Match::must(1, 15..16)), it.next());"] # [doc = " assert_eq!(Some(Match::must(0, 17..21)), it.next());"] # [doc = " assert_eq!(None, it.next());"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn new_many_sparse < P : AsRef < str > > (patterns : & [P] ,) -> Result < Regex < sparse :: DFA < Vec < u8 > > > , BuildError > { Builder :: new () . build_many_sparse (patterns) } }
    };
}

impl_104!()
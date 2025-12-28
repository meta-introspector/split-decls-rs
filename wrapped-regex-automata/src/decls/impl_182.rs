macro_rules! deps {
    () => {
        ByteSet!();
        MatchKind!();
        Config!();
        State!();
        StateMap!();
        NFA!();
        BuildError!();
        DFA!();
        Runner!();
        OwnedDFA!();
        SparseSets!();
        StateBuilderEmpty!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl Config { # [doc = " Create a new default config for a determinizer. The determinizer may be"] # [doc = " configured before calling `run`."] pub fn new () -> Config { Config { match_kind : MatchKind :: LeftmostFirst , quit : ByteSet :: empty () , dfa_size_limit : None , determinize_size_limit : None , } } # [doc = " Run determinization on the given NFA and write the resulting DFA into"] # [doc = " the one given. The DFA given should be initialized but otherwise empty."] # [doc = " \"Initialized\" means that it is setup to handle the NFA's byte classes,"] # [doc = " number of patterns and whether to build start states for each pattern."] pub fn run (& self , nfa : & thompson :: NFA , dfa : & mut dense :: OwnedDFA ,) -> Result < () , BuildError > { let dead = State :: dead () ; let quit = State :: dead () ; let mut cache = StateMap :: default () ; cache . insert (dead . clone () , DEAD) ; let runner = Runner { config : self . clone () , nfa , dfa , builder_states : alloc :: vec ! [dead , quit] , cache , memory_usage_state : 0 , sparses : SparseSets :: new (nfa . states () . len ()) , stack : alloc :: vec ! [] , scratch_state_builder : StateBuilderEmpty :: new () , } ; runner . run () } # [doc = " The match semantics to use for determinization."] # [doc = ""] # [doc = " MatchKind::All corresponds to the standard textbook construction."] # [doc = " All possible match states are represented in the DFA."] # [doc = " MatchKind::LeftmostFirst permits greediness and otherwise tries to"] # [doc = " simulate the match semantics of backtracking regex engines. Namely,"] # [doc = " only a subset of match states are built, and dead states are used to"] # [doc = " stop searches with an unanchored prefix."] # [doc = ""] # [doc = " The default is MatchKind::LeftmostFirst."] pub fn match_kind (& mut self , kind : MatchKind) -> & mut Config { self . match_kind = kind ; self } # [doc = " The set of bytes to use that will cause the DFA to enter a quit state,"] # [doc = " stop searching and return an error. By default, this is empty."] pub fn quit (& mut self , set : ByteSet) -> & mut Config { self . quit = set ; self } # [doc = " The limit, in bytes of the heap, that the DFA is permitted to use. This"] # [doc = " does not include the auxiliary heap storage used by determinization."] pub fn dfa_size_limit (& mut self , bytes : Option < usize >) -> & mut Config { self . dfa_size_limit = bytes ; self } # [doc = " The limit, in bytes of the heap, that determinization itself is allowed"] # [doc = " to use. This does not include the size of the DFA being built."] pub fn determinize_size_limit (& mut self , bytes : Option < usize > ,) -> & mut Config { self . determinize_size_limit = bytes ; self } }
    };
}

impl_182!();
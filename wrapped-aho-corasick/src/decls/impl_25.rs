macro_rules! deps {
    () => {
        Automaton!();
        AcAutomaton!();
        Match!();
        MatchKind!();
        PatternID!();
        Anchored!();
        Prefilter!();
        Input!();
        StateID!();
        OverlappingState!();
        MatchError!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [doc (hidden)] unsafe impl Automaton for Arc < dyn AcAutomaton > { # [inline (always)] fn start_state (& self , anchored : Anchored) -> Result < StateID , MatchError > { (* * self) . start_state (anchored) } # [inline (always)] fn next_state (& self , anchored : Anchored , sid : StateID , byte : u8 ,) -> StateID { (* * self) . next_state (anchored , sid , byte) } # [inline (always)] fn is_special (& self , sid : StateID) -> bool { (* * self) . is_special (sid) } # [inline (always)] fn is_dead (& self , sid : StateID) -> bool { (* * self) . is_dead (sid) } # [inline (always)] fn is_match (& self , sid : StateID) -> bool { (* * self) . is_match (sid) } # [inline (always)] fn is_start (& self , sid : StateID) -> bool { (* * self) . is_start (sid) } # [inline (always)] fn match_kind (& self) -> MatchKind { (* * self) . match_kind () } # [inline (always)] fn match_len (& self , sid : StateID) -> usize { (* * self) . match_len (sid) } # [inline (always)] fn match_pattern (& self , sid : StateID , index : usize) -> PatternID { (* * self) . match_pattern (sid , index) } # [inline (always)] fn patterns_len (& self) -> usize { (* * self) . patterns_len () } # [inline (always)] fn pattern_len (& self , pid : PatternID) -> usize { (* * self) . pattern_len (pid) } # [inline (always)] fn min_pattern_len (& self) -> usize { (* * self) . min_pattern_len () } # [inline (always)] fn max_pattern_len (& self) -> usize { (* * self) . max_pattern_len () } # [inline (always)] fn memory_usage (& self) -> usize { (* * self) . memory_usage () } # [inline (always)] fn prefilter (& self) -> Option < & Prefilter > { (* * self) . prefilter () } # [inline (always)] fn try_find (& self , input : & Input < '_ > ,) -> Result < Option < Match > , MatchError > { (* * self) . try_find (input) } # [inline (always)] fn try_find_overlapping (& self , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { (* * self) . try_find_overlapping (input , state) } }
    };
}

impl_25!();
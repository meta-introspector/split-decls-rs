macro_rules! deps {
    () => {
        StateID!();
        State!();
        Match!();
        MatchKind!();
        SmallIndex!();
        MatchError!();
        Prefilter!();
        NFA!();
        PatternID!();
        Transition!();
        Automaton!();
        Anchored!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        unsafe impl Automaton for NFA { # [inline (always)] fn start_state (& self , anchored : Anchored) -> Result < StateID , MatchError > { match anchored { Anchored :: No => Ok (self . special . start_unanchored_id) , Anchored :: Yes => Ok (self . special . start_anchored_id) , } } # [inline (always)] fn next_state (& self , anchored : Anchored , mut sid : StateID , byte : u8 ,) -> StateID { loop { let next = self . follow_transition (sid , byte) ; if next != NFA :: FAIL { return next ; } if anchored . is_anchored () { return NFA :: DEAD ; } sid = self . states [sid] . fail () ; } } # [inline (always)] fn is_special (& self , sid : StateID) -> bool { sid <= self . special . max_special_id } # [inline (always)] fn is_dead (& self , sid : StateID) -> bool { sid == NFA :: DEAD } # [inline (always)] fn is_match (& self , sid : StateID) -> bool { ! self . is_dead (sid) && sid <= self . special . max_match_id } # [inline (always)] fn is_start (& self , sid : StateID) -> bool { sid == self . special . start_unanchored_id || sid == self . special . start_anchored_id } # [inline (always)] fn match_kind (& self) -> MatchKind { self . match_kind } # [inline (always)] fn patterns_len (& self) -> usize { self . pattern_lens . len () } # [inline (always)] fn pattern_len (& self , pid : PatternID) -> usize { self . pattern_lens [pid] . as_usize () } # [inline (always)] fn min_pattern_len (& self) -> usize { self . min_pattern_len } # [inline (always)] fn max_pattern_len (& self) -> usize { self . max_pattern_len } # [inline (always)] fn match_len (& self , sid : StateID) -> usize { self . iter_matches (sid) . count () } # [inline (always)] fn match_pattern (& self , sid : StateID , index : usize) -> PatternID { self . iter_matches (sid) . nth (index) . unwrap () } # [inline (always)] fn memory_usage (& self) -> usize { self . states . len () * core :: mem :: size_of :: < State > () + self . sparse . len () * core :: mem :: size_of :: < Transition > () + self . matches . len () * core :: mem :: size_of :: < Match > () + self . dense . len () * StateID :: SIZE + self . pattern_lens . len () * SmallIndex :: SIZE + self . prefilter . as_ref () . map_or (0 , | p | p . memory_usage ()) } # [inline (always)] fn prefilter (& self) -> Option < & Prefilter > { self . prefilter . as_ref () } }
    };
}

impl_83!()
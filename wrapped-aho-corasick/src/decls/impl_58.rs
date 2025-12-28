macro_rules! deps {
    () => {
        DFA!();
        PatternID!();
        Anchored!();
        SmallIndex!();
        Prefilter!();
        Automaton!();
        StateID!();
        MatchError!();
        MatchKind!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        unsafe impl Automaton for DFA { # [inline (always)] fn start_state (& self , anchored : Anchored) -> Result < StateID , MatchError > { match anchored { Anchored :: No => { let start = self . special . start_unanchored_id ; if start == DFA :: DEAD { Err (MatchError :: invalid_input_unanchored ()) } else { Ok (start) } } Anchored :: Yes => { let start = self . special . start_anchored_id ; if start == DFA :: DEAD { Err (MatchError :: invalid_input_anchored ()) } else { Ok (start) } } } } # [inline (always)] fn next_state (& self , _anchored : Anchored , sid : StateID , byte : u8 ,) -> StateID { let class = self . byte_classes . get (byte) ; self . trans [(sid . as_u32 () + u32 :: from (class)) . as_usize ()] } # [inline (always)] fn is_special (& self , sid : StateID) -> bool { sid <= self . special . max_special_id } # [inline (always)] fn is_dead (& self , sid : StateID) -> bool { sid == DFA :: DEAD } # [inline (always)] fn is_match (& self , sid : StateID) -> bool { ! self . is_dead (sid) && sid <= self . special . max_match_id } # [inline (always)] fn is_start (& self , sid : StateID) -> bool { sid == self . special . start_unanchored_id || sid == self . special . start_anchored_id } # [inline (always)] fn match_kind (& self) -> MatchKind { self . match_kind } # [inline (always)] fn patterns_len (& self) -> usize { self . pattern_lens . len () } # [inline (always)] fn pattern_len (& self , pid : PatternID) -> usize { self . pattern_lens [pid] . as_usize () } # [inline (always)] fn min_pattern_len (& self) -> usize { self . min_pattern_len } # [inline (always)] fn max_pattern_len (& self) -> usize { self . max_pattern_len } # [inline (always)] fn match_len (& self , sid : StateID) -> usize { debug_assert ! (self . is_match (sid)) ; let offset = (sid . as_usize () >> self . stride2) - 2 ; self . matches [offset] . len () } # [inline (always)] fn match_pattern (& self , sid : StateID , index : usize) -> PatternID { debug_assert ! (self . is_match (sid)) ; let offset = (sid . as_usize () >> self . stride2) - 2 ; self . matches [offset] [index] } # [inline (always)] fn memory_usage (& self) -> usize { use core :: mem :: size_of ; (self . trans . len () * size_of :: < u32 > ()) + (self . matches . len () * size_of :: < Vec < PatternID > > ()) + self . matches_memory_usage + (self . pattern_lens . len () * size_of :: < SmallIndex > ()) + self . prefilter . as_ref () . map_or (0 , | p | p . memory_usage ()) } # [inline (always)] fn prefilter (& self) -> Option < & Prefilter > { self . prefilter . as_ref () } }
    };
}

impl_58!();
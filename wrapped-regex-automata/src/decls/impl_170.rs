macro_rules! deps {
    () => {
        MatchError!();
        Input!();
        HalfMatch!();
        Prefilter!();
        Automaton!();
        PatternID!();
        StartError!();
        Anchored!();
        PatternSet!();
        Config!();
        OverlappingState!();
        StateID!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        unsafe impl < 'a , A : Automaton + ? Sized > Automaton for & 'a A { # [inline] fn next_state (& self , current : StateID , input : u8) -> StateID { (* * self) . next_state (current , input) } # [inline] unsafe fn next_state_unchecked (& self , current : StateID , input : u8 ,) -> StateID { (* * self) . next_state_unchecked (current , input) } # [inline] fn next_eoi_state (& self , current : StateID) -> StateID { (* * self) . next_eoi_state (current) } # [inline] fn start_state (& self , config : & start :: Config ,) -> Result < StateID , StartError > { (* * self) . start_state (config) } # [inline] fn start_state_forward (& self , input : & Input < '_ > ,) -> Result < StateID , MatchError > { (* * self) . start_state_forward (input) } # [inline] fn start_state_reverse (& self , input : & Input < '_ > ,) -> Result < StateID , MatchError > { (* * self) . start_state_reverse (input) } # [inline] fn universal_start_state (& self , mode : Anchored) -> Option < StateID > { (* * self) . universal_start_state (mode) } # [inline] fn is_special_state (& self , id : StateID) -> bool { (* * self) . is_special_state (id) } # [inline] fn is_dead_state (& self , id : StateID) -> bool { (* * self) . is_dead_state (id) } # [inline] fn is_quit_state (& self , id : StateID) -> bool { (* * self) . is_quit_state (id) } # [inline] fn is_match_state (& self , id : StateID) -> bool { (* * self) . is_match_state (id) } # [inline] fn is_start_state (& self , id : StateID) -> bool { (* * self) . is_start_state (id) } # [inline] fn is_accel_state (& self , id : StateID) -> bool { (* * self) . is_accel_state (id) } # [inline] fn pattern_len (& self) -> usize { (* * self) . pattern_len () } # [inline] fn match_len (& self , id : StateID) -> usize { (* * self) . match_len (id) } # [inline] fn match_pattern (& self , id : StateID , index : usize) -> PatternID { (* * self) . match_pattern (id , index) } # [inline] fn has_empty (& self) -> bool { (* * self) . has_empty () } # [inline] fn is_utf8 (& self) -> bool { (* * self) . is_utf8 () } # [inline] fn is_always_start_anchored (& self) -> bool { (* * self) . is_always_start_anchored () } # [inline] fn accelerator (& self , id : StateID) -> & [u8] { (* * self) . accelerator (id) } # [inline] fn get_prefilter (& self) -> Option < & Prefilter > { (* * self) . get_prefilter () } # [inline] fn try_search_fwd (& self , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , MatchError > { (* * self) . try_search_fwd (input) } # [inline] fn try_search_rev (& self , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , MatchError > { (* * self) . try_search_rev (input) } # [inline] fn try_search_overlapping_fwd (& self , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { (* * self) . try_search_overlapping_fwd (input , state) } # [inline] fn try_search_overlapping_rev (& self , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { (* * self) . try_search_overlapping_rev (input , state) } # [cfg (feature = "alloc")] # [inline] fn try_which_overlapping_matches (& self , input : & Input < '_ > , patset : & mut PatternSet ,) -> Result < () , MatchError > { (* * self) . try_which_overlapping_matches (input , patset) } }
    };
}

impl_170!();
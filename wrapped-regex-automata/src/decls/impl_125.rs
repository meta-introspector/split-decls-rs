macro_rules! deps {
    () => {
        StateID!();
        PatternID!();
        Start!();
        Prefilter!();
        Automaton!();
        Anchored!();
        Config!();
        DFA!();
        StartError!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        unsafe impl < T : AsRef < [u8] > > Automaton for DFA < T > { # [inline] fn is_special_state (& self , id : StateID) -> bool { self . special . is_special_state (id) } # [inline] fn is_dead_state (& self , id : StateID) -> bool { self . special . is_dead_state (id) } # [inline] fn is_quit_state (& self , id : StateID) -> bool { self . special . is_quit_state (id) } # [inline] fn is_match_state (& self , id : StateID) -> bool { self . special . is_match_state (id) } # [inline] fn is_start_state (& self , id : StateID) -> bool { self . special . is_start_state (id) } # [inline] fn is_accel_state (& self , id : StateID) -> bool { self . special . is_accel_state (id) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn next_state (& self , current : StateID , input : u8) -> StateID { let input = self . tt . classes . get (input) ; self . tt . state (current) . next (input) } # [inline] unsafe fn next_state_unchecked (& self , current : StateID , input : u8 ,) -> StateID { self . next_state (current , input) } # [inline] fn next_eoi_state (& self , current : StateID) -> StateID { self . tt . state (current) . next_eoi () } # [inline] fn pattern_len (& self) -> usize { self . tt . pattern_len } # [inline] fn match_len (& self , id : StateID) -> usize { self . tt . state (id) . pattern_len () } # [inline] fn match_pattern (& self , id : StateID , match_index : usize) -> PatternID { if self . tt . pattern_len == 1 { return PatternID :: ZERO ; } self . tt . state (id) . pattern_id (match_index) } # [inline] fn has_empty (& self) -> bool { self . flags . has_empty } # [inline] fn is_utf8 (& self) -> bool { self . flags . is_utf8 } # [inline] fn is_always_start_anchored (& self) -> bool { self . flags . is_always_start_anchored } # [inline] fn start_state (& self , config : & start :: Config ,) -> Result < StateID , StartError > { let anchored = config . get_anchored () ; let start = match config . get_look_behind () { None => Start :: Text , Some (byte) => { if ! self . quitset . is_empty () && self . quitset . contains (byte) { return Err (StartError :: quit (byte)) ; } self . st . start_map . get (byte) } } ; self . st . start (anchored , start) } # [inline] fn universal_start_state (& self , mode : Anchored) -> Option < StateID > { match mode { Anchored :: No => self . st . universal_start_unanchored , Anchored :: Yes => self . st . universal_start_anchored , Anchored :: Pattern (_) => None , } } # [inline] fn accelerator (& self , id : StateID) -> & [u8] { self . tt . state (id) . accelerator () } # [inline] fn get_prefilter (& self) -> Option < & Prefilter > { self . pre . as_ref () } }
    };
}

impl_125!()
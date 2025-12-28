macro_rules! deps {
    () => {
        PatternID!();
        StateID!();
        StateBuilderEmpty!();
        LookSet!();
        Repr!();
        State!();
        ReprVec!();
    };
}

macro_rules! impl_855 {
    () => {
        deps!();
        # [doc = " For docs on these routines, see the internal Repr and ReprVec types below."] impl State { pub (crate) fn dead () -> State { StateBuilderEmpty :: new () . into_matches () . into_nfa () . to_state () } pub (crate) fn is_match (& self) -> bool { self . repr () . is_match () } pub (crate) fn is_from_word (& self) -> bool { self . repr () . is_from_word () } pub (crate) fn is_half_crlf (& self) -> bool { self . repr () . is_half_crlf () } pub (crate) fn look_have (& self) -> LookSet { self . repr () . look_have () } pub (crate) fn look_need (& self) -> LookSet { self . repr () . look_need () } pub (crate) fn match_len (& self) -> usize { self . repr () . match_len () } pub (crate) fn match_pattern (& self , index : usize) -> PatternID { self . repr () . match_pattern (index) } pub (crate) fn match_pattern_ids (& self) -> Option < Vec < PatternID > > { self . repr () . match_pattern_ids () } # [cfg (all (test , not (miri)))] pub (crate) fn iter_match_pattern_ids < F : FnMut (PatternID) > (& self , f : F) { self . repr () . iter_match_pattern_ids (f) } pub (crate) fn iter_nfa_state_ids < F : FnMut (StateID) > (& self , f : F) { self . repr () . iter_nfa_state_ids (f) } pub (crate) fn memory_usage (& self) -> usize { self . 0 . len () } fn repr (& self) -> Repr < '_ > { Repr (& self . 0) } }
    };
}

impl_855!()
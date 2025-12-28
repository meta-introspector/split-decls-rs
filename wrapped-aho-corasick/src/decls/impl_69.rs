macro_rules! deps {
    () => {
        State!();
        StateID!();
        NFA!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl core :: fmt :: Debug for NFA { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: automaton :: fmt_state_indicator ; writeln ! (f , "contiguous::NFA(") ? ; let mut sid = NFA :: DEAD ; loop { let raw = & self . repr [sid . as_usize () ..] ; if raw . is_empty () { break ; } let is_match = self . is_match (sid) ; let state = State :: read (self . alphabet_len , is_match , raw) ; fmt_state_indicator (f , self , sid) ? ; write ! (f , "{:06}({:06}): " , sid . as_usize () , state . fail . as_usize ()) ? ; state . fmt (f) ? ; write ! (f , "\n") ? ; if self . is_match (sid) { write ! (f , "         matches: ") ? ; for i in 0 .. state . match_len { let pid = State :: match_pattern (self . alphabet_len , raw , i) ; if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{}" , pid . as_usize ()) ? ; } write ! (f , "\n") ? ; } if sid == NFA :: DEAD { writeln ! (f , "F {:06}:" , NFA :: FAIL . as_usize ()) ? ; } let len = State :: len (self . alphabet_len , is_match , raw) ; sid = StateID :: new (sid . as_usize () . checked_add (len) . unwrap ()) . unwrap () ; } writeln ! (f , "match kind: {:?}" , self . match_kind) ? ; writeln ! (f , "prefilter: {:?}" , self . prefilter . is_some ()) ? ; writeln ! (f , "state length: {:?}" , self . state_len) ? ; writeln ! (f , "pattern length: {:?}" , self . patterns_len ()) ? ; writeln ! (f , "shortest pattern length: {:?}" , self . min_pattern_len) ? ; writeln ! (f , "longest pattern length: {:?}" , self . max_pattern_len) ? ; writeln ! (f , "alphabet length: {:?}" , self . alphabet_len) ? ; writeln ! (f , "byte classes: {:?}" , self . byte_classes) ? ; writeln ! (f , "memory usage: {:?}" , self . memory_usage ()) ? ; writeln ! (f , ")") ? ; Ok (()) } }
    };
}

impl_69!()
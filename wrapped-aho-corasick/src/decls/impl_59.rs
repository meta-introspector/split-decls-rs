macro_rules! deps {
    () => {
        DFA!();
        StateID!();
        DebugByte!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl core :: fmt :: Debug for DFA { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use crate :: { automaton :: { fmt_state_indicator , sparse_transitions } , util :: debug :: DebugByte , } ; writeln ! (f , "dfa::DFA(") ? ; for index in 0 .. self . state_len { let sid = StateID :: new_unchecked (index << self . stride2) ; if index == 1 { writeln ! (f , "F {:06}:" , sid . as_usize ()) ? ; continue ; } fmt_state_indicator (f , self , sid) ? ; write ! (f , "{:06}: " , sid . as_usize ()) ? ; let it = (0 .. self . byte_classes . alphabet_len ()) . map (| class | { (class . as_u8 () , self . trans [sid . as_usize () + class]) }) ; for (i , (start , end , next)) in sparse_transitions (it) . enumerate () { if i > 0 { write ! (f , ", ") ? ; } if start == end { write ! (f , "{:?} => {:?}" , DebugByte (start) , next . as_usize ()) ? ; } else { write ! (f , "{:?}-{:?} => {:?}" , DebugByte (start) , DebugByte (end) , next . as_usize ()) ? ; } } write ! (f , "\n") ? ; if self . is_match (sid) { write ! (f , " matches: ") ? ; for i in 0 .. self . match_len (sid) { if i > 0 { write ! (f , ", ") ? ; } let pid = self . match_pattern (sid , i) ; write ! (f , "{}" , pid . as_usize ()) ? ; } write ! (f , "\n") ? ; } } writeln ! (f , "match kind: {:?}" , self . match_kind) ? ; writeln ! (f , "prefilter: {:?}" , self . prefilter . is_some ()) ? ; writeln ! (f , "state length: {:?}" , self . state_len) ? ; writeln ! (f , "pattern length: {:?}" , self . patterns_len ()) ? ; writeln ! (f , "shortest pattern length: {:?}" , self . min_pattern_len) ? ; writeln ! (f , "longest pattern length: {:?}" , self . max_pattern_len) ? ; writeln ! (f , "alphabet length: {:?}" , self . alphabet_len) ? ; writeln ! (f , "stride: {:?}" , 1 << self . stride2) ? ; writeln ! (f , "byte classes: {:?}" , self . byte_classes) ? ; writeln ! (f , "memory usage: {:?}" , self . memory_usage ()) ? ; writeln ! (f , ")") ? ; Ok (()) } }
    };
}

impl_59!();
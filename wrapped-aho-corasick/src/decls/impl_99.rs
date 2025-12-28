macro_rules! deps {
    () => {
        NFA!();
        DebugByte!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl core :: fmt :: Debug for NFA { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { use crate :: { automaton :: { fmt_state_indicator , sparse_transitions } , util :: debug :: DebugByte , } ; writeln ! (f , "noncontiguous::NFA(") ? ; for (sid , state) in self . states . iter () . with_state_ids () { if sid == NFA :: FAIL { writeln ! (f , "F {:06}:" , sid . as_usize ()) ? ; continue ; } fmt_state_indicator (f , self , sid) ? ; write ! (f , "{:06}({:06}): " , sid . as_usize () , state . fail . as_usize ()) ? ; let it = sparse_transitions (self . iter_trans (sid) . map (| t | (t . byte , t . next)) ,) . enumerate () ; for (i , (start , end , sid)) in it { if i > 0 { write ! (f , ", ") ? ; } if start == end { write ! (f , "{:?} => {:?}" , DebugByte (start) , sid . as_usize ()) ? ; } else { write ! (f , "{:?}-{:?} => {:?}" , DebugByte (start) , DebugByte (end) , sid . as_usize ()) ? ; } } write ! (f , "\n") ? ; if self . is_match (sid) { write ! (f , "         matches: ") ? ; for (i , pid) in self . iter_matches (sid) . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{}" , pid . as_usize ()) ? ; } write ! (f , "\n") ? ; } } writeln ! (f , "match kind: {:?}" , self . match_kind) ? ; writeln ! (f , "prefilter: {:?}" , self . prefilter . is_some ()) ? ; writeln ! (f , "state length: {:?}" , self . states . len ()) ? ; writeln ! (f , "pattern length: {:?}" , self . patterns_len ()) ? ; writeln ! (f , "shortest pattern length: {:?}" , self . min_pattern_len) ? ; writeln ! (f , "longest pattern length: {:?}" , self . max_pattern_len) ? ; writeln ! (f , "memory usage: {:?}" , self . memory_usage ()) ? ; writeln ! (f , ")") ? ; Ok (()) } }
    };
}

impl_99!();
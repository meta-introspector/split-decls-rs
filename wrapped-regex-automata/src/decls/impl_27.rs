macro_rules! deps {
    () => {
        DFA!();
        Anchored!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T : AsRef < [u32] > > fmt :: Debug for DFA < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "dense::DFA(") ? ; for state in self . states () { fmt_state_indicator (f , self , state . id ()) ? ; let id = if f . alternate () { state . id () . as_usize () } else { self . to_index (state . id ()) } ; write ! (f , "{id:06?}: ") ? ; state . fmt (f) ? ; write ! (f , "\n") ? ; } writeln ! (f , "") ? ; for (i , (start_id , anchored , sty)) in self . starts () . enumerate () { let id = if f . alternate () { start_id . as_usize () } else { self . to_index (start_id) } ; if i % self . st . stride == 0 { match anchored { Anchored :: No => writeln ! (f , "START-GROUP(unanchored)") ? , Anchored :: Yes => writeln ! (f , "START-GROUP(anchored)") ? , Anchored :: Pattern (pid) => { writeln ! (f , "START_GROUP(pattern: {pid:?})") ? } } } writeln ! (f , "  {sty:?} => {id:06?}") ? ; } if self . pattern_len () > 1 { writeln ! (f , "") ? ; for i in 0 .. self . ms . len () { let id = self . ms . match_state_id (self , i) ; let id = if f . alternate () { id . as_usize () } else { self . to_index (id) } ; write ! (f , "MATCH({id:06?}): ") ? ; for (i , & pid) in self . ms . pattern_id_slice (i) . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{pid:?}") ? ; } writeln ! (f , "") ? ; } } writeln ! (f , "state length: {:?}" , self . state_len ()) ? ; writeln ! (f , "pattern length: {:?}" , self . pattern_len ()) ? ; writeln ! (f , "flags: {:?}" , self . flags) ? ; writeln ! (f , ")") ? ; Ok (()) } }
    };
}

impl_27!();
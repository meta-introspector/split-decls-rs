macro_rules! deps {
    () => {
        NFA!();
        DebugByte!();
        State!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a > core :: fmt :: Debug for State < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { use crate :: { automaton :: sparse_transitions , util :: debug :: DebugByte } ; let it = sparse_transitions (self . transitions ()) . filter (| & (_ , _ , sid) | sid != NFA :: FAIL) . enumerate () ; for (i , (start , end , sid)) in it { if i > 0 { write ! (f , ", ") ? ; } if start == end { write ! (f , "{:?} => {:?}" , DebugByte (start) , sid . as_usize ()) ? ; } else { write ! (f , "{:?}-{:?} => {:?}" , DebugByte (start) , DebugByte (end) , sid . as_usize ()) ? ; } } Ok (()) } }
    };
}

impl_73!()
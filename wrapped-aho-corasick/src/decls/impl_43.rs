macro_rules! deps {
    () => {
        Match!();
        StreamFindIter!();
        Automaton!();
        StreamChunk!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a , A : Automaton , R : std :: io :: Read > Iterator for StreamFindIter < 'a , A , R > { type Item = std :: io :: Result < Match > ; fn next (& mut self) -> Option < std :: io :: Result < Match > > { loop { match self . it . next () { None => return None , Some (Err (err)) => return Some (Err (err)) , Some (Ok (StreamChunk :: NonMatch { .. })) => { } Some (Ok (StreamChunk :: Match { mat , .. })) => { return Some (Ok (mat)) ; } } } } }
    };
}

impl_43!()
macro_rules! deps {
    () => {
        Transition!();
        StateChunksIter!();
        State!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl State { # [doc = " Mark this state as a match state and freeze the active chunk such that"] # [doc = " it can not be further mutated."] fn add_match (& mut self) { if self . transitions . is_empty () && ! self . chunks . is_empty () { return ; } let chunk_start = self . active_chunk_start () ; let chunk_end = self . transitions . len () ; self . chunks . push ((chunk_start , chunk_end)) ; } # [doc = " Returns true if and only if this state is a leaf state. That is, a"] # [doc = " state that has no outgoing transitions."] fn is_leaf (& self) -> bool { self . transitions . is_empty () } # [doc = " Returns an iterator over all of the chunks (including the currently"] # [doc = " active chunk) in this state. Since the active chunk is included, the"] # [doc = " iterator is guaranteed to always yield at least one chunk (although the"] # [doc = " chunk may be empty)."] fn chunks (& self) -> StateChunksIter < '_ > { StateChunksIter { transitions : & * self . transitions , chunks : self . chunks . iter () , active : Some (self . active_chunk ()) , } } # [doc = " Returns the active chunk as a slice of transitions."] fn active_chunk (& self) -> & [Transition] { let start = self . active_chunk_start () ; & self . transitions [start ..] } # [doc = " Returns the index into 'transitions' where the active chunk starts."] fn active_chunk_start (& self) -> usize { self . chunks . last () . map_or (0 , | & (_ , end) | end) } }
    };
}

impl_494!();
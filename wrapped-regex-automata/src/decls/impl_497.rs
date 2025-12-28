macro_rules! deps {
    () => {
        StateChunksIter!();
        Transition!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl < 'a > Iterator for StateChunksIter < 'a > { type Item = & 'a [Transition] ; fn next (& mut self) -> Option < & 'a [Transition] > { if let Some (& (start , end)) = self . chunks . next () { return Some (& self . transitions [start .. end]) ; } if let Some (chunk) = self . active . take () { return Some (chunk) ; } None } }
    };
}

impl_497!()
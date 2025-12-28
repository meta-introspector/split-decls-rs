macro_rules! deps {
    () => {
        Match!();
        StreamFindIter!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a , R : std :: io :: Read > Iterator for StreamFindIter < 'a , R > { type Item = Result < Match , std :: io :: Error > ; fn next (& mut self) -> Option < Result < Match , std :: io :: Error > > { self . 0 . next () } }
    };
}

impl_18!()
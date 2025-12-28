macro_rules! deps {
    () => {
        PatternID!();
        PatternSetIter!();
    };
}

macro_rules! impl_928 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a > DoubleEndedIterator for PatternSetIter < 'a > { fn next_back (& mut self) -> Option < PatternID > { while let Some ((index , & yes)) = self . it . next_back () { if yes { return Some (PatternID :: new_unchecked (index)) ; } } None } }
    };
}

impl_928!()
macro_rules! deps {
    () => {
        PatternID!();
        PatternSetIter!();
    };
}

macro_rules! impl_927 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a > Iterator for PatternSetIter < 'a > { type Item = PatternID ; fn next (& mut self) -> Option < PatternID > { while let Some ((index , & yes)) = self . it . next () { if yes { return Some (PatternID :: new_unchecked (index)) ; } } None } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
    };
}

impl_927!()
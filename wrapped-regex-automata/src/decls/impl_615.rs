macro_rules! deps {
    () => {
        Span!();
        CapturesPatternIter!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        impl < 'a > Iterator for CapturesPatternIter < 'a > { type Item = Option < Span > ; fn next (& mut self) -> Option < Option < Span > > { let (group_index , _) = self . names . next () ? ; Some (self . caps . get_group (group_index)) } fn size_hint (& self) -> (usize , Option < usize >) { self . names . size_hint () } fn count (self) -> usize { self . names . count () } }
    };
}

impl_615!()
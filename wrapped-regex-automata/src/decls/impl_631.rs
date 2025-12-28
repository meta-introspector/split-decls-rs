macro_rules! deps {
    () => {
        GroupInfoPatternNames!();
    };
}

macro_rules! impl_631 {
    () => {
        deps!();
        impl < 'a > Iterator for GroupInfoPatternNames < 'a > { type Item = Option < & 'a str > ; fn next (& mut self) -> Option < Option < & 'a str > > { self . it . next () . map (| x | x . as_deref ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } fn count (self) -> usize { self . it . count () } }
    };
}

impl_631!()
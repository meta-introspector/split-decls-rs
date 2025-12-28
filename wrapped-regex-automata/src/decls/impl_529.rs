macro_rules! deps {
    () => {
        PatternID!();
        PatternIter!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl < 'a > Iterator for PatternIter < 'a > { type Item = PatternID ; fn next (& mut self) -> Option < PatternID > { self . it . next () } }
    };
}

impl_529!()
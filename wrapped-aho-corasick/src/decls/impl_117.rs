macro_rules! deps {
    () => {
        Match!();
        FindIter!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < 's , 'h > Iterator for FindIter < 's , 'h > { type Item = Match ; fn next (& mut self) -> Option < Match > { if self . span . start > self . span . end { return None ; } match self . searcher . find_in (self . haystack , self . span) { None => None , Some (m) => { self . span . start = m . end () ; Some (m) } } } }
    };
}

impl_117!();
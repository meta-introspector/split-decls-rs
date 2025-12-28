macro_rules! deps {
    () => {
        IdIterator!();
    };
}

macro_rules! impl_1018 {
    () => {
        deps!();
        impl < S : BuildHasher > Iterator for IdIterator < '_ , S > { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { let current = { if self . current . is_none () { self . current = Some (0) ; self . current . as_mut () . unwrap () } else { let current = self . current . as_mut () . unwrap () ; * current += 1 ; current } } ; while self . removed_ids . contains (current) && * current < self . upper_bound { * current += 1 ; } if * current < self . upper_bound { Some (* current) } else { None } } }
    };
}

impl_1018!();
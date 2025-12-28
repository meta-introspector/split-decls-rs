macro_rules! deps {
    () => {
        LocationRangeIter!();
        Location!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'ctx , R > Iterator for LocationRangeIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { type Item = (u64 , u64 , Location < 'ctx >) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . next_loc () . unwrap_or_default () } }
    };
}

impl_90!();
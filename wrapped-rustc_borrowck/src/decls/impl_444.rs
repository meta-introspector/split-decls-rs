macro_rules! deps {
    () => {
        AppearancesIter!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < 'a > Iterator for AppearancesIter < 'a > { type Item = AppearanceIndex ; fn next (& mut self) -> Option < AppearanceIndex > { if let Some (c) = self . current { self . current = self . appearances [c] . next ; Some (c) } else { None } } }
    };
}

impl_444!();
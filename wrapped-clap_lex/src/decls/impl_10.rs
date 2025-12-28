macro_rules! deps {
    () => {
        ShortFlags!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 's > Iterator for ShortFlags < 's > { type Item = Result < char , & 's OsStr > ; fn next (& mut self) -> Option < Self :: Item > { self . next_flag () } }
    };
}

impl_10!()
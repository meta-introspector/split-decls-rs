macro_rules! deps {
    () => {
        NaiveDateWeeksIterator!();
        Item!();
        Days!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl DoubleEndedIterator for NaiveDateWeeksIterator { fn next_back (& mut self) -> Option < Self :: Item > { let current = self . value ; self . value = current . checked_sub_days (Days :: new (7)) ? ; Some (current) } }
    };
}

impl_359!()
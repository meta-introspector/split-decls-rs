macro_rules! deps {
    () => {
        NaiveDateDaysIterator!();
        Item!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl DoubleEndedIterator for NaiveDateDaysIterator { fn next_back (& mut self) -> Option < Self :: Item > { let current = self . value ; self . value = current . pred_opt () ? ; Some (current) } }
    };
}

impl_354!()
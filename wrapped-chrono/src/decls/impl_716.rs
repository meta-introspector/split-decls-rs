macro_rules! deps {
    () => {
        Item!();
        WeekdaySetIter!();
    };
}

macro_rules! impl_716 {
    () => {
        deps!();
        impl DoubleEndedIterator for WeekdaySetIter { fn next_back (& mut self) -> Option < Self :: Item > { if self . days . is_empty () { return None ; } let (before , after) = self . days . split_at (self . start) ; let days = if before . is_empty () { after } else { before } ; let next_back = days . last () . expect ("the collection is not empty") ; self . days . remove (next_back) ; Some (next_back) } }
    };
}

impl_716!();
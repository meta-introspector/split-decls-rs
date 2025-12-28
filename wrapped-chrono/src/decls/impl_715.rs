macro_rules! deps {
    () => {
        Item!();
        Weekday!();
        WeekdaySetIter!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl Iterator for WeekdaySetIter { type Item = Weekday ; fn next (& mut self) -> Option < Self :: Item > { if self . days . is_empty () { return None ; } let (before , after) = self . days . split_at (self . start) ; let days = if after . is_empty () { before } else { after } ; let next = days . first () . expect ("the collection is not empty") ; self . days . remove (next) ; Some (next) } }
    };
}

impl_715!();
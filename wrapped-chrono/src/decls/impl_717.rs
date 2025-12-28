macro_rules! deps {
    () => {
        WeekdaySetIter!();
    };
}

macro_rules! impl_717 {
    () => {
        deps!();
        impl ExactSizeIterator for WeekdaySetIter { fn len (& self) -> usize { self . days . len () . into () } }
    };
}

impl_717!();
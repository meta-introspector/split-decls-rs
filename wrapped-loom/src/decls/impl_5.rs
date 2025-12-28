macro_rules! deps {
    () => {
        LocationSet!();
        Set!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl LocationSet { pub (super) fn new () -> LocationSet { LocationSet { locations : Default :: default () , } } pub (super) fn track (& mut self , location : Location , threads : & thread :: Set) { let active_id = threads . active_id () ; self . locations [active_id . as_usize ()] = location ; } }
    };
}

impl_5!();
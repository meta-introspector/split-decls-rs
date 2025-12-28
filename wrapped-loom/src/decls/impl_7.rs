macro_rules! deps {
    () => {
        Set!();
        LocationSet!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl ops :: Index < & thread :: Set > for LocationSet { type Output = Location ; fn index (& self , threads : & thread :: Set) -> & Location { let active_id = threads . active_id () ; self . locations . index (active_id . as_usize ()) } }
    };
}

impl_7!()
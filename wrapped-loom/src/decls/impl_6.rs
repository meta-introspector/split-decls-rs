macro_rules! deps {
    () => {
        LocationSet!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl ops :: Index < usize > for LocationSet { type Output = Location ; fn index (& self , index : usize) -> & Location { self . locations . index (index) } }
    };
}

impl_6!()
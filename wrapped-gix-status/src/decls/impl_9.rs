macro_rules! deps {
    () => {
        EntryStatus!();
        Change!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T , U > From < Change < T , U > > for EntryStatus < T , U > { fn from (value : Change < T , U >) -> Self { EntryStatus :: Change (value) } }
    };
}

impl_9!();
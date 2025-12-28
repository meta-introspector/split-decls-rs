macro_rules! deps {
    () => {
        Event!();
        Header!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a > From < Header < 'a > > for Event < 'a > { fn from (header : Header < '_ >) -> Event < '_ > { Event :: SectionHeader (header) } }
    };
}

impl_167!();
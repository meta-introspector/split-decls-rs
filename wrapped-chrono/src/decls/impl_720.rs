macro_rules! deps {
    () => {
        Weekday!();
        WeekdaySet!();
        Item!();
    };
}

macro_rules! impl_720 {
    () => {
        deps!();
        impl FromIterator < Weekday > for WeekdaySet { fn from_iter < T : IntoIterator < Item = Weekday > > (iter : T) -> Self { iter . into_iter () . map (Self :: single) . fold (Self :: EMPTY , Self :: union) } }
    };
}

impl_720!()
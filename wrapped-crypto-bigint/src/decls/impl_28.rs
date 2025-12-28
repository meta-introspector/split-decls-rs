macro_rules! deps {
    () => {
        Checked!();
        CheckedAdd!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > Add < & Checked < T > > for & Checked < T > where T : CheckedAdd + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn add (self , rhs : & Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_add (& rhs))) ,) } }
    };
}

impl_28!();
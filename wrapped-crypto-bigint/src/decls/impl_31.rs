macro_rules! deps {
    () => {
        Checked!();
        CheckedSub!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > Sub < Checked < T > > for & Checked < T > where T : CheckedSub + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn sub (self , rhs : Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_sub (& rhs))) ,) } }
    };
}

impl_31!();
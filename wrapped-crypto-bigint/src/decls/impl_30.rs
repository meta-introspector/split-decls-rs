macro_rules! deps {
    () => {
        CheckedSub!();
        Checked!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T > Sub < & Self > for Checked < T > where T : CheckedSub + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn sub (self , rhs : & Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_sub (& rhs))) ,) } }
    };
}

impl_30!();
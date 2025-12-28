macro_rules! deps {
    () => {
        CheckedDiv!();
        Checked!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > Div < & Self > for Checked < T > where T : CheckedDiv + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn div (self , rhs : & Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_div (& rhs))) ,) } }
    };
}

impl_38!();
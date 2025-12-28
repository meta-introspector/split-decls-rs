macro_rules! deps {
    () => {
        Checked!();
        CheckedMul!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > Mul < Checked < T > > for & Checked < T > where T : CheckedMul + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn mul (self , rhs : Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_mul (& rhs))) ,) } }
    };
}

impl_35!();
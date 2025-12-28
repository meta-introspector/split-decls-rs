macro_rules! deps {
    () => {
        CheckedMul!();
        Checked!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T > Mul < & Checked < T > > for & Checked < T > where T : CheckedMul + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn mul (self , rhs : & Checked < T >) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_mul (& rhs))) ,) } }
    };
}

impl_36!();
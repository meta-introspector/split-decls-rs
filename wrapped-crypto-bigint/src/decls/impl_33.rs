macro_rules! deps {
    () => {
        Checked!();
        CheckedMul!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > Mul < Self > for Checked < T > where T : CheckedMul + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn mul (self , rhs : Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_mul (& rhs))) ,) } }
    };
}

impl_33!();
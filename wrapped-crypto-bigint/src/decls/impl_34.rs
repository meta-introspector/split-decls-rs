macro_rules! deps {
    () => {
        CheckedMul!();
        Checked!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > Mul < & Self > for Checked < T > where T : CheckedMul + ConditionallySelectable + Default , { type Output = Checked < T > ; # [inline] fn mul (self , rhs : & Self) -> Self :: Output { Checked (self . 0 . and_then (| lhs | rhs . 0 . and_then (| rhs | lhs . checked_mul (& rhs))) ,) } }
    };
}

impl_34!()
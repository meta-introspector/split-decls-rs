macro_rules! deps {
    () => {
        WideningMul!();
        ConcatenatingMul!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        # [allow (deprecated)] impl < T , Rhs > WideningMul < Rhs > for T where T : ConcatenatingMul < Rhs > , { type Output = < T as ConcatenatingMul < Rhs > > :: Output ; fn widening_mul (& self , rhs : Rhs) -> Self :: Output { self . concatenating_mul (rhs) } }
    };
}

impl_321!();
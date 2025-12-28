macro_rules! ScalarMul {
    () => {
        # [doc = " A helper trait for types implementing group scalar multiplication."] pub trait ScalarMul < Rhs , Output = Self > : Mul < Rhs , Output = Output > + MulAssign < Rhs > { }
    };
}

ScalarMul!();
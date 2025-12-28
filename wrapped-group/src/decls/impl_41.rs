macro_rules! deps {
    () => {
        Group!();
        WnafScalar!();
        WnafBase!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < G : Group , const WINDOW_SIZE : usize > Mul < & WnafScalar < G :: Scalar , WINDOW_SIZE > > for & WnafBase < G , WINDOW_SIZE > { type Output = G ; fn mul (self , rhs : & WnafScalar < G :: Scalar , WINDOW_SIZE >) -> Self :: Output { wnaf_exp (& self . table , & rhs . wnaf) } }
    };
}

impl_41!();
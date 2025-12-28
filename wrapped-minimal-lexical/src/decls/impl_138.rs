macro_rules! deps {
    () => {
        StackVec!();
        Limb!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl ops :: MulAssign < & [bigint :: Limb] > for StackVec { # [inline] fn mul_assign (& mut self , rhs : & [bigint :: Limb]) { bigint :: large_mul (self , rhs) . unwrap () ; } }
    };
}

impl_138!()
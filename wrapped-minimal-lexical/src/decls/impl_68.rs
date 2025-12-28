macro_rules! deps {
    () => {
        HeapVec!();
        Limb!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl ops :: MulAssign < & [bigint :: Limb] > for HeapVec { # [inline] fn mul_assign (& mut self , rhs : & [bigint :: Limb]) { bigint :: large_mul (self , rhs) . unwrap () ; } }
    };
}

impl_68!();
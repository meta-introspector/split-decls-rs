macro_rules! deps {
    () => {
        Bigint!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ops :: MulAssign < & Bigint > for Bigint { fn mul_assign (& mut self , rhs : & Bigint) { self . data *= & rhs . data ; } }
    };
}

impl_16!()
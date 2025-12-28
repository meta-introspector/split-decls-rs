macro_rules! deps {
    () => {
        Hash64!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl BitXorAssign < u64 > for Hash64 { # [inline] fn bitxor_assign (& mut self , rhs : u64) { self . inner ^= rhs ; } }
    };
}

impl_2!()
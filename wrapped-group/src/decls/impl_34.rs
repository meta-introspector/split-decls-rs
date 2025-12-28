macro_rules! deps {
    () => {
        Group!();
        Wnaf!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < B , S : AsMut < Vec < i64 > > > Wnaf < usize , B , S > { # [doc = " Performs exponentiation given a scalar."] pub fn scalar < G : Group > (& mut self , scalar : & < G as Group > :: Scalar) -> G where B : AsRef < [G] > , { wnaf_form (self . scalar . as_mut () , scalar . to_repr () , self . window_size) ; wnaf_exp (self . base . as_ref () , self . scalar . as_mut ()) } }
    };
}

impl_34!()
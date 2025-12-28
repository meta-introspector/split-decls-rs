macro_rules! deps {
    () => {
        Group!();
        Wnaf!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < B , S : AsRef < [i64] > > Wnaf < usize , B , S > { # [doc = " Performs exponentiation given a base."] pub fn base < G : Group > (& mut self , base : G) -> G where B : AsMut < Vec < G > > , { wnaf_table (self . base . as_mut () , base , self . window_size) ; wnaf_exp (self . base . as_mut () , self . scalar . as_ref ()) } }
    };
}

impl_33!()
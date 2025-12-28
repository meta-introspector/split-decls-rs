macro_rules! deps {
    () => {
        WnafGroup!();
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < G : WnafGroup > Wnaf < () , Vec < G > , Vec < i64 > > { # [doc = " Given a base and a number of scalars, compute a window table and return a `Wnaf` object that"] # [doc = " can perform exponentiations with `.scalar(..)`."] pub fn base (& mut self , base : G , num_scalars : usize) -> Wnaf < usize , & [G] , & mut Vec < i64 > > { let window_size = G :: recommended_wnaf_for_num_scalars (num_scalars) ; wnaf_table (& mut self . base , base , window_size) ; Wnaf { base : & self . base [..] , scalar : & mut self . scalar , window_size , } } # [doc = " Given a scalar, compute its wNAF representation and return a `Wnaf` object that can perform"] # [doc = " exponentiations with `.base(..)`."] pub fn scalar (& mut self , scalar : & < G as Group > :: Scalar) -> Wnaf < usize , & mut Vec < G > , & [i64] > { let window_size = 4 ; wnaf_form (& mut self . scalar , scalar . to_repr () , window_size) ; Wnaf { base : & mut self . base , scalar : & self . scalar [..] , window_size , } } }
    };
}

impl_28!();
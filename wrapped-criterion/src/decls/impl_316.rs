macro_rules! deps {
    () => {
        Distribution!();
        TupledDistributionsBuilder!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < A > TupledDistributionsBuilder for (Vec < A > ,) where A : Copy , { type Item = (A ,) ; fn new (size : usize) -> (Vec < A > ,) { (Vec :: with_capacity (size) ,) } fn push (& mut self , tuple : (A ,)) { (self . 0) . push (tuple . 0) ; } fn extend (& mut self , other : & mut (Vec < A > ,)) { (self . 0) . append (& mut other . 0) ; } fn complete (self) -> (Distribution < A > ,) { (Distribution (self . 0 . into_boxed_slice ()) ,) } }
    };
}

impl_316!();
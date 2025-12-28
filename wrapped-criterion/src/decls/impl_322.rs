macro_rules! deps {
    () => {
        TupledDistributionsBuilder!();
        Distribution!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < A , B , C > TupledDistributionsBuilder for (Vec < A > , Vec < B > , Vec < C >) where A : Copy , B : Copy , C : Copy , { type Item = (A , B , C) ; fn new (size : usize) -> (Vec < A > , Vec < B > , Vec < C >) { (Vec :: with_capacity (size) , Vec :: with_capacity (size) , Vec :: with_capacity (size) ,) } fn push (& mut self , tuple : (A , B , C)) { (self . 0) . push (tuple . 0) ; (self . 1) . push (tuple . 1) ; (self . 2) . push (tuple . 2) ; } fn extend (& mut self , other : & mut (Vec < A > , Vec < B > , Vec < C >)) { (self . 0) . append (& mut other . 0) ; (self . 1) . append (& mut other . 1) ; (self . 2) . append (& mut other . 2) ; } fn complete (self) -> (Distribution < A > , Distribution < B > , Distribution < C >) { (Distribution (self . 0 . into_boxed_slice ()) , Distribution (self . 1 . into_boxed_slice ()) , Distribution (self . 2 . into_boxed_slice ()) ,) } }
    };
}

impl_322!()
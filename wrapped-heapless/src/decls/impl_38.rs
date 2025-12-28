macro_rules! deps {
    () => {
        DequeInner!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T : fmt :: Debug , S : VecStorage < T > + ? Sized > fmt :: Debug for DequeInner < T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self) . finish () } }
    };
}

impl_38!()
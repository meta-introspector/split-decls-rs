macro_rules! deps {
    () => {
        VacantEntry!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < T : fmt :: Debug , S , A : Allocator > fmt :: Debug for VacantEntry < '_ , T , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . get ()) . finish () } }
    };
}

impl_461!();
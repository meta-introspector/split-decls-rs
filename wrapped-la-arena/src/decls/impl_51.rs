macro_rules! deps {
    () => {
        Arena!();
        Idx!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T > Index < Idx < T > > for Arena < T > { type Output = T ; fn index (& self , idx : Idx < T >) -> & T { let idx = idx . into_raw () . 0 as usize ; & self . data [idx] } }
    };
}

impl_51!()
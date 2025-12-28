macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > Drop for VecInner < T , LenT , S > { fn drop (& mut self) { let mut_slice = self . as_mut_slice () ; unsafe { ptr :: drop_in_place (mut_slice) } } }
    };
}

impl_293!()
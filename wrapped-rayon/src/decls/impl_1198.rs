macro_rules! deps {
    () => {
        InsertionHole!();
    };
}

macro_rules! impl_1198 {
    () => {
        deps!();
        impl < T > Drop for InsertionHole < T > { fn drop (& mut self) { unsafe { ptr :: copy_nonoverlapping (self . src , self . dest , 1) ; } } }
    };
}

impl_1198!()
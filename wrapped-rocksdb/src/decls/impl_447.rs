macro_rules! deps {
    () => {
        DB!();
        Transaction!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        unsafe impl < DB > Send for Transaction < '_ , DB > { }
    };
}

impl_447!()
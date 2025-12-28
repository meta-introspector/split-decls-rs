macro_rules! deps {
    () => {
        NotifyWaker!();
    };
}

macro_rules! impl_1025 {
    () => {
        deps!();
        impl Notify01 for NotifyWaker { fn notify (& self , _ : usize) { self . 0 . wake_by_ref () ; } }
    };
}

impl_1025!()
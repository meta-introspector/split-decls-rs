macro_rules! deps {
    () => {
        SenderTask!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl SenderTask { fn new () -> Self { Self { task : None , is_parked : false } } fn notify (& mut self) { self . is_parked = false ; if let Some (task) = self . task . take () { task . wake () ; } } }
    };
}

impl_62!();
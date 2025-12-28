macro_rules! deps {
    () => {
        WaiterSignaler!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        unsafe impl Send for WaiterSignaler { }
    };
}

impl_123!()
macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! WaiterSignaler {
    () => {
        deps!();
        pub struct WaiterSignaler (HANDLE) ;
    };
}

WaiterSignaler!();
macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! SharedErrorSlot {
    () => {
        deps!();
        pub (crate) type SharedErrorSlot = Arc < parking_lot :: Mutex < Option < entry :: Error > > > ;
    };
}

SharedErrorSlot!();
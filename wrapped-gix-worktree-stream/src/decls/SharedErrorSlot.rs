macro_rules! SharedErrorSlot {
    () => {
        pub (crate) type SharedErrorSlot = Arc < parking_lot :: Mutex < Option < entry :: Error > > > ;
    };
}

SharedErrorSlot!()
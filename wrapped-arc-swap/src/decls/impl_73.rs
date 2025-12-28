macro_rules! deps {
    () => {
        NodeReservation!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Drop for NodeReservation < '_ > { fn drop (& mut self) { self . 0 . active_writers . fetch_sub (1 , Release) ; } }
    };
}

impl_73!()
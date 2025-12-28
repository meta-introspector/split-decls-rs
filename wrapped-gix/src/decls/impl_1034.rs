macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_1034 {
    () => {
        deps!();
        # [cfg (feature = "parallel")] impl Drop for Iter { fn drop (& mut self) { crate :: util :: parallel_iter_drop (self . rx_and_join . take () , & self . should_interrupt) ; } }
    };
}

impl_1034!()
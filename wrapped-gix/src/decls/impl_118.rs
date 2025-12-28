macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        # [cfg (feature = "parallel")] impl Drop for Iter { fn drop (& mut self) { crate :: util :: parallel_iter_drop (self . rx_and_join . take () . map (| (rx , handle) | (rx , handle , None :: < std :: thread :: JoinHandle < () > >)) , & self . should_interrupt ,) ; } }
    };
}

impl_118!()
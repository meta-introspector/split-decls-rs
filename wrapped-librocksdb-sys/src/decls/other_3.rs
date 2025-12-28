macro_rules! other_3 {
    () => {
        # [cfg (feature = "jemalloc")] extern crate tikv_jemalloc_sys ;
    };
}

other_3!()
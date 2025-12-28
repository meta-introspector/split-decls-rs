macro_rules! decimal {
    () => {
        # [cfg (feature = "decimal")] mod decimal ;
    };
}

decimal!()
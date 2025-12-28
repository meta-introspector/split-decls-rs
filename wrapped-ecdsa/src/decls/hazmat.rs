macro_rules! hazmat {
    () => {
        # [cfg (feature = "hazmat")] pub mod hazmat ;
    };
}

hazmat!();
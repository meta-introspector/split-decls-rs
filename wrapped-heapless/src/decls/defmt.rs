macro_rules! defmt {
    () => {
        # [cfg (feature = "defmt")] mod defmt ;
    };
}

defmt!();
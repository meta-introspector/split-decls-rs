macro_rules! deps {
    () => {
        Task!();
    };
}

macro_rules! Current {
    () => {
        deps!();
        # [derive (Clone)] struct Current (task01 :: Task) ;
    };
}

Current!()
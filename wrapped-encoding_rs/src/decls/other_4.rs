macro_rules! other_4 {
    () => {
        # [cfg (all (test , feature = "serde"))] extern crate bincode ;
    };
}

other_4!();
macro_rules! other_6 {
    () => {
        # [cfg (all (test , feature = "serde"))] extern crate serde_json ;
    };
}

other_6!();
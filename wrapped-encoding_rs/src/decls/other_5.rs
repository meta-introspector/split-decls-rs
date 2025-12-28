macro_rules! other_5 {
    () => {
        # [cfg (all (test , feature = "serde"))] # [macro_use] extern crate serde_derive ;
    };
}

other_5!();
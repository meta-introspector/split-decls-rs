macro_rules! other_0 {
    () => {
        # [cfg (all (feature = "alloc" , not (feature = "std")))] extern crate alloc ;
    };
}

other_0!()
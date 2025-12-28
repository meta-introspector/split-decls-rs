macro_rules! signing {
    () => {
        # [cfg (feature = "algorithm")] mod signing ;
    };
}

signing!();
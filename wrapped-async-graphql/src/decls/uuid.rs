macro_rules! uuid {
    () => {
        # [cfg (feature = "uuid-validator")] mod uuid ;
    };
}

uuid!();
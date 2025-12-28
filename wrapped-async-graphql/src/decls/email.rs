macro_rules! email {
    () => {
        # [cfg (feature = "email-validator")] mod email ;
    };
}

email!();
macro_rules! unlock_notify {
    () => {
        # [cfg (feature = "unlock_notify")] mod unlock_notify ;
    };
}

unlock_notify!()
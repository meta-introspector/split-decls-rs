macro_rules! poll_timeout {
    () => {
        # [cfg (any (feature = "poll" , feature = "event"))] mod poll_timeout ;
    };
}

poll_timeout!()
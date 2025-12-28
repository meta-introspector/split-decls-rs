macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! LogAllRefUpdates {
    () => {
        deps!();
        # [doc = " The `core.logAllRefUpdates` key."] pub type LogAllRefUpdates = keys :: Any < validate :: LogAllRefUpdates > ;
    };
}

LogAllRefUpdates!()
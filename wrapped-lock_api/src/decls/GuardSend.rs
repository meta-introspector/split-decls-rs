macro_rules! GuardSend {
    () => {
        # [doc = " Marker type which indicates that the Guard type for a lock is `Send`."] pub struct GuardSend (()) ;
    };
}

GuardSend!();
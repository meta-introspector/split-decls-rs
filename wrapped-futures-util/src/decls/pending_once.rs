macro_rules! deps {
    () => {
        PendingOnce!();
    };
}

macro_rules! pending_once {
    () => {
        deps!();
        # [doc (hidden)] pub fn pending_once () -> PendingOnce { PendingOnce { is_ready : false } }
    };
}

pending_once!()
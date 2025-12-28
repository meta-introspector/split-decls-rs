macro_rules! PendingOnce {
    () => {
        # [allow (missing_debug_implementations)] # [doc (hidden)] pub struct PendingOnce { is_ready : bool , }
    };
}

PendingOnce!();
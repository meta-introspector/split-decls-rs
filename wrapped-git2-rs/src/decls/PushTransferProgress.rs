macro_rules! PushTransferProgress {
    () => {
        # [doc = " Callback for push transfer progress"] # [doc = ""] # [doc = " Parameters:"] # [doc = " * current"] # [doc = " * total"] # [doc = " * bytes"] pub type PushTransferProgress < 'a > = dyn FnMut (usize , usize , usize) + 'a ;
    };
}

PushTransferProgress!();
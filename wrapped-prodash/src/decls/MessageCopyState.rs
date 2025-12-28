macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! MessageCopyState {
    () => {
        deps!();
        # [doc = " State used to keep track of what's new since the last time message were copied."] # [doc = ""] # [doc = " Note that due to the nature of a ring buffer, there is no guarantee that you see all messages."] pub struct MessageCopyState { cursor : usize , buf_len : usize , total : usize , }
    };
}

MessageCopyState!()
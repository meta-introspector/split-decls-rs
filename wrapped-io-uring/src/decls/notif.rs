macro_rules! notif {
    () => {
        # [doc = " Returns whether this completion event is a notification."] # [doc = ""] # [doc = " This corresponds to the `IORING_CQE_F_NOTIF` flag,"] # [doc = " currently used by the [SendZc](crate::opcode::SendZc) operation."] pub fn notif (flags : u32) -> bool { flags & sys :: IORING_CQE_F_NOTIF != 0 }
    };
}

notif!();
macro_rules! buffer_more {
    () => {
        # [doc = " Return whether the buffer will be reused by future CQE completions"] # [doc = ""] # [doc = " This corresponds to the `IORING_CQE_BUF_MORE` flag, and it signals to"] # [doc = " the consumer that it should expect further completions involging the"] # [doc = " related buffer ID when the registered buffer ring was setup with"] # [doc = " the `IOU_PBUF_RING_INC` flag."] pub fn buffer_more (flags : u32) -> bool { flags & sys :: IORING_CQE_F_BUF_MORE != 0 }
    };
}

buffer_more!();
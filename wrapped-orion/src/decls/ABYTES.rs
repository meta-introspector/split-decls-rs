macro_rules! ABYTES {
    () => {
        # [doc = " Size of additional data appended to each message."] pub const ABYTES : usize = POLY1305_OUTSIZE + TAG_SIZE ;
    };
}

ABYTES!()
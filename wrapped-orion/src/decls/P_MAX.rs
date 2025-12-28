macro_rules! P_MAX {
    () => {
        # [doc = " The maximum size of the plaintext (see [RFC 8439](https://www.rfc-editor.org/rfc/rfc8439#section-2.8))."] pub const P_MAX : u64 = (u32 :: MAX as u64) * 64 ;
    };
}

P_MAX!()
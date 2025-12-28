macro_rules! macro_253 {
    () => {
        ffi_fn ! { # [doc = " Get the length of the bytes this buffer contains."] fn hyper_buf_len (buf : * const hyper_buf) -> size_t { unsafe { (* buf) . 0 . len () } } }
    };
}

macro_253!();
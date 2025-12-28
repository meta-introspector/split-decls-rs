macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl Drop for Buf { fn drop (& mut self) { unsafe { raw :: git_buf_dispose (& mut self . raw) } } }
    };
}

impl_242!();
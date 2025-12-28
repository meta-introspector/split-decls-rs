macro_rules! deps {
    () => {
        Entry!();
        Fixed!();
        Fd!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        opcode ! { # [doc = " Close a file descriptor, equivalent to `close(2)`."] # [doc = ""] # [doc = " Use a types::Fixed(fd) argument to close an io_uring direct descriptor."] pub struct Close { fd : { impl sealed :: UseFixed } , ;; } pub const CODE = sys :: IORING_OP_CLOSE ; pub fn build (self) -> Entry { let Close { fd } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; match fd { sealed :: Target :: Fd (fd) => sqe . fd = fd , sealed :: Target :: Fixed (idx) => { sqe . fd = 0 ; sqe . __bindgen_anon_5 . file_index = idx + 1 ; } } Entry (sqe) } }
    };
}

macro_63!()
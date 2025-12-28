macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_41 {
    () => {
        deps!();
        opcode ! { # [doc = " Do not perform any I/O."] # [doc = ""] # [doc = " This is useful for testing the performance of the io_uring implementation itself."] # [derive (Debug)] pub struct Nop { ;; } pub const CODE = sys :: IORING_OP_NOP ; pub fn build (self) -> Entry { let Nop { } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . fd = - 1 ; Entry (sqe) } }
    };
}

macro_41!()
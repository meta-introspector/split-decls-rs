macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_87 {
    () => {
        deps!();
        opcode ! { # [doc = " Get extended attribute from a file descriptor, equivalent to `fgetxattr(2)`."] pub struct FGetXattr { fd : { impl sealed :: UseFixed } , name : { * const libc :: c_char } , value : { * mut libc :: c_void } , len : { u32 } , ;; } pub const CODE = sys :: IORING_OP_FGETXATTR ; pub fn build (self) -> Entry { let FGetXattr { fd , name , value , len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_2 . addr = name as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = value as _ ; sqe . __bindgen_anon_3 . xattr_flags = 0 ; Entry (sqe) } }
    };
}

macro_87!()
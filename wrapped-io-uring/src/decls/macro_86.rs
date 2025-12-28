macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_86 {
    () => {
        deps!();
        opcode ! { # [doc = " Set extended attribute, equivalent to `setxattr(2)`."] pub struct SetXattr { name : { * const libc :: c_char } , value : { * const libc :: c_void } , path : { * const libc :: c_char } , len : { u32 } , ;; flags : i32 = 0 } pub const CODE = sys :: IORING_OP_SETXATTR ; pub fn build (self) -> Entry { let SetXattr { name , value , path , flags , len } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; sqe . __bindgen_anon_2 . addr = name as _ ; sqe . len = len ; sqe . __bindgen_anon_1 . off = value as _ ; unsafe { sqe . __bindgen_anon_6 . __bindgen_anon_1 . as_mut () . addr3 = path as _ } ; sqe . __bindgen_anon_3 . xattr_flags = flags as _ ; Entry (sqe) } }
    };
}

macro_86!()
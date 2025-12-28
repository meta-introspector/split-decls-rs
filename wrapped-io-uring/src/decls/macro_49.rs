macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_49 {
    () => {
        deps!();
        opcode ! { # [doc = " Sync a file segment with disk, equivalent to `sync_file_range(2)`."] # [derive (Debug)] pub struct SyncFileRange { fd : { impl sealed :: UseFixed } , len : { u32 } , ;; # [doc = " the offset method holds the offset in bytes"] offset : u64 = 0 , # [doc = " the flags method holds the flags for the command"] flags : u32 = 0 } pub const CODE = sys :: IORING_OP_SYNC_FILE_RANGE ; pub fn build (self) -> Entry { let SyncFileRange { fd , len , offset , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . len = len ; sqe . __bindgen_anon_1 . off = offset ; sqe . __bindgen_anon_3 . sync_range_flags = flags ; Entry (sqe) } }
    };
}

macro_49!();
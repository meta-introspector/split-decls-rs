macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! macro_44 {
    () => {
        deps!();
        opcode ! { # [doc = " File sync, equivalent to `fsync(2)`."] # [doc = ""] # [doc = " Note that, while I/O is initiated in the order in which it appears in the submission queue,"] # [doc = " completions are unordered. For example, an application which places a write I/O followed by"] # [doc = " an fsync in the submission queue cannot expect the fsync to apply to the write. The two"] # [doc = " operations execute in parallel, so the fsync may complete before the write is issued to the"] # [doc = " storage. The same is also true for previously issued writes that have not completed prior to"] # [doc = " the fsync."] # [derive (Debug)] pub struct Fsync { fd : { impl sealed :: UseFixed } , ;; # [doc = " The `flags` bit mask may contain either 0, for a normal file integrity sync,"] # [doc = " or [types::FsyncFlags::DATASYNC] to provide data sync only semantics."] # [doc = " See the descriptions of `O_SYNC` and `O_DSYNC` in the `open(2)` manual page for more information."] flags : types :: FsyncFlags = types :: FsyncFlags :: empty () } pub const CODE = sys :: IORING_OP_FSYNC ; pub fn build (self) -> Entry { let Fsync { fd , flags } = self ; let mut sqe = sqe_zeroed () ; sqe . opcode = Self :: CODE ; assign_fd ! (sqe . fd = fd) ; sqe . __bindgen_anon_3 . fsync_flags = flags . bits () ; Entry (sqe) } }
    };
}

macro_44!();
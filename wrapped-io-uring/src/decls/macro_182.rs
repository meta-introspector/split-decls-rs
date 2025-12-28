macro_rules! macro_182 {
    () => {
        bitflags ! { # [doc = " Options for [`Fsync`](super::Fsync)."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub struct FsyncFlags : u32 { const DATASYNC = sys :: IORING_FSYNC_DATASYNC ; } }
    };
}

macro_182!()
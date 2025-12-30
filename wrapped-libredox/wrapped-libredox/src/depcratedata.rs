// Generated macro for data (module)
macro_rules! Depcratedata {
() => {
// Module: crate
// Provides: {"data"}
// Dependencies: {}
pub mod data { pub use libc :: iovec as IoVec ; pub use libc :: sigaction as SigAction ; pub use libc :: stat as Stat ; pub use libc :: statvfs as StatVfs ; pub use libc :: timespec as TimeSpec ; pub fn timespec_from_mut_bytes (bytes : & mut [u8]) -> & mut TimeSpec { assert ! (bytes . len () >= core :: mem :: size_of ::< TimeSpec > ()) ; assert_eq ! (bytes . as_ptr () as usize % core :: mem :: align_of ::< TimeSpec > () , 0) ; unsafe { & mut * bytes . as_mut_ptr () . cast () } } pub fn timespec_from_bytes (bytes : & [u8]) -> & TimeSpec { assert ! (bytes . len () >= core :: mem :: size_of ::< TimeSpec > ()) ; assert_eq ! (bytes . as_ptr () as usize % core :: mem :: align_of ::< TimeSpec > () , 0) ; unsafe { & * bytes . as_ptr () . cast () } } # [cfg (target_os = "redox")] pub use libc :: sigset_t as SigSet ; # [cfg (not (target_os = "redox"))] pub type SigSet = u64 ; }
};
}

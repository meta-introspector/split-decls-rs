// Generated macro for write (function)
macro_rules! Depcrate_syscallswrite {
() => {
// Module: crate::syscalls
// Provides: {"write"}
// Dependencies: {}
unsafe fn write (fd : FileDescriptor , buf : * const u8 , len : usize) -> isize { let slice = unsafe { core :: slice :: from_raw_parts (buf , len) } ; crate :: fd :: write (fd , slice) . map_or_else (| e | isize :: try_from (- i32 :: from (e)) . unwrap () , | v | v . try_into () . unwrap () ,) }
};
}

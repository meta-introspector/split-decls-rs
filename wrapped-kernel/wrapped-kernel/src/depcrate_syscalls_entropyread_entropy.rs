// Generated macro for read_entropy (function)
macro_rules! Depcrate_syscalls_entropyread_entropy {
() => {
// Module: crate::syscalls::entropy
// Provides: {"read_entropy"}
// Dependencies: {}
unsafe fn read_entropy (buf : * mut u8 , len : usize , flags : u32) -> isize { let Some (flags) = Flags :: from_bits (flags) else { return - i32 :: from (Errno :: Inval) as isize ; } ; let buf = unsafe { let len = usize :: min (len , isize :: MAX as usize) ; buf . write_bytes (0 , len) ; slice :: from_raw_parts_mut (buf , len) } ; let ret = entropy :: read (buf , flags) ; if ret < 0 { warn ! ("Unable to read entropy! Fallback to a naive implementation!") ; for i in & mut * buf { * i = (generate_park_miller_lehmer_random_number () & 0xff) . try_into () . unwrap () ; } buf . len () . try_into () . unwrap () } else { ret } }
};
}

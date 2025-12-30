// Generated macro for recvmmsg (function)
macro_rules! Depcrate_mmsgrecvmmsg {
() => {
// Module: crate::mmsg
// Provides: {"recvmmsg"}
// Dependencies: {}
pub fn recvmmsg (fd : BorrowedFd , bufs : & mut [ReadBuf < '_ >]) -> io :: Result < usize > { let mut msgvec : SmallVec < [libc :: mmsghdr ; MAX_MMSG] > = SmallVec :: new () ; let mut slices : SmallVec < [IoSlice ; MAX_MMSG] > = SmallVec :: new () ; let mut ret = 0 ; for bufs in bufs . chunks_mut (MAX_MMSG) { msgvec . clear () ; slices . clear () ; for buf in bufs . iter_mut () { let b = unsafe { & mut * (buf . unfilled_mut () as * mut [std :: mem :: MaybeUninit < u8 >] as * mut [u8]) } ; slices . push (IoSlice :: new (b)) ; msgvec . push (libc :: mmsghdr { msg_hdr : libc :: msghdr { msg_name : std :: ptr :: null_mut () , msg_namelen : 0 , msg_iov : slices . last_mut () . unwrap () as * mut _ as * mut _ , msg_iovlen : 1 , msg_control : std :: ptr :: null_mut () , msg_controllen : 0 , msg_flags : 0 , } , msg_len : buf . capacity () . try_into () . unwrap () , }) ; } let result = unsafe { libc :: recvmmsg (fd . as_raw_fd () , msgvec . as_mut_ptr () , msgvec . len () as _ , 0 , std :: ptr :: null_mut () ,) } ; if result == - 1 { break ; } for i in 0 .. result as usize { let filled = msgvec [i] . msg_len as usize ; unsafe { bufs [i] . assume_init (filled) } ; bufs [i] . advance (filled) ; ret += 1 ; } if (result as usize) < MAX_MMSG { break ; } } if ret == 0 { return Err (io :: Error :: last_os_error ()) ; } Ok (ret) }
};
}

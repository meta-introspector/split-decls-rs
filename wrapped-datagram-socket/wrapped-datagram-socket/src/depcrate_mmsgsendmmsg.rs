// Generated macro for sendmmsg (function)
macro_rules! Depcrate_mmsgsendmmsg {
() => {
// Module: crate::mmsg
// Provides: {"sendmmsg"}
// Dependencies: {}
pub fn sendmmsg (fd : BorrowedFd , bufs : & [ReadBuf < '_ >]) -> io :: Result < usize > { let mut msgvec : SmallVec < [libc :: mmsghdr ; MAX_MMSG] > = SmallVec :: new () ; let mut slices : SmallVec < [IoSlice ; MAX_MMSG] > = SmallVec :: new () ; let mut ret = 0 ; for bufs in bufs . chunks (MAX_MMSG) { msgvec . clear () ; slices . clear () ; for buf in bufs . iter () { slices . push (IoSlice :: new (buf . filled ())) ; msgvec . push (libc :: mmsghdr { msg_hdr : libc :: msghdr { msg_name : std :: ptr :: null_mut () , msg_namelen : 0 , msg_iov : slices . last_mut () . unwrap () as * mut _ as * mut _ , msg_iovlen : 1 , msg_control : std :: ptr :: null_mut () , msg_controllen : 0 , msg_flags : 0 , } , msg_len : buf . capacity () . try_into () . unwrap () , }) ; } let result = unsafe { libc :: sendmmsg (fd . as_raw_fd () , msgvec . as_mut_ptr () , msgvec . len () as _ , 0 ,) } ; if result == - 1 { break ; } ret += result as usize ; if (result as usize) < MAX_MMSG { break ; } } if ret == 0 { return Err (io :: Error :: last_os_error ()) ; } Ok (ret) }
};
}

macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Mmap { # [doc = " Map `len` bytes starting from the offset `offset` in the file descriptor `fd` into memory."] pub fn new (fd : & OwnedFd , offset : libc :: off_t , len : usize) -> io :: Result < Mmap > { unsafe { match libc :: mmap (ptr :: null_mut () , len , libc :: PROT_READ | libc :: PROT_WRITE , libc :: MAP_SHARED | libc :: MAP_POPULATE , fd . as_raw_fd () , offset ,) { libc :: MAP_FAILED => Err (io :: Error :: last_os_error ()) , addr => { let addr = ptr :: NonNull :: new_unchecked (addr) ; Ok (Mmap { addr , len }) } } } } # [doc = " Do not make the stored memory accessible by child processes after a `fork`."] pub fn dontfork (& self) -> io :: Result < () > { match unsafe { libc :: madvise (self . addr . as_ptr () , self . len , libc :: MADV_DONTFORK) } { 0 => Ok (()) , _ => Err (io :: Error :: last_os_error ()) , } } # [doc = " Get a pointer to the memory."] # [inline] pub fn as_mut_ptr (& self) -> * mut libc :: c_void { self . addr . as_ptr () } # [doc = " Get a pointer to the data at the given offset."] # [inline] pub unsafe fn offset (& self , offset : u32) -> * mut libc :: c_void { self . as_mut_ptr () . add (offset as usize) } }
    };
}

impl_2!()
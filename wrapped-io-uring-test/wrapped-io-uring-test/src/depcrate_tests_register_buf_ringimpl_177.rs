// Generated macro for impl_177 (impl)
macro_rules! Depcrate_tests_register_buf_ringimpl_177 {
() => {
// Module: crate::tests::register_buf_ring
// Provides: {"impl_177"}
// Dependencies: {}
impl AnonymousMmap { # [doc = " Allocate `len` bytes that are page aligned and zero-filled."] pub fn new (len : usize) -> io :: Result < AnonymousMmap > { unsafe { match libc :: mmap (ptr :: null_mut () , len , libc :: PROT_READ | libc :: PROT_WRITE , libc :: MAP_ANONYMOUS | libc :: MAP_SHARED | libc :: MAP_POPULATE , - 1 , 0 ,) { libc :: MAP_FAILED => Err (io :: Error :: last_os_error ()) , addr => { let addr = ptr :: NonNull :: new_unchecked (addr) ; Ok (AnonymousMmap { addr , len }) } } } } # [doc = " Do not make the stored memory accessible by child processes after a `fork`."] pub fn dontfork (& self) -> io :: Result < () > { match unsafe { libc :: madvise (self . addr . as_ptr () , self . len , libc :: MADV_DONTFORK) } { 0 => Ok (()) , _ => Err (io :: Error :: last_os_error ()) , } } # [doc = " Get a pointer to the memory."] # [inline] pub fn as_ptr (& self) -> * const libc :: c_void { self . addr . as_ptr () } # [doc = " Get a mut pointer to the memory."] # [inline] pub fn as_ptr_mut (& self) -> * mut libc :: c_void { self . addr . as_ptr () } # [doc = " Get a pointer to the data at the given offset."] # [inline] # [allow (dead_code)] pub unsafe fn offset (& self , offset : u32) -> * const libc :: c_void { self . as_ptr () . add (offset as usize) } # [doc = " Get a mut pointer to the data at the given offset."] # [inline] # [allow (dead_code)] pub unsafe fn offset_mut (& self , offset : u32) -> * mut libc :: c_void { self . as_ptr_mut () . add (offset as usize) } }
};
}

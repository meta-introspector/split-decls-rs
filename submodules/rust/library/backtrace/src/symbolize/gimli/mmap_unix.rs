mkuse!{use super :: mystd :: fs :: File ;}
mkuse!{use super :: mystd :: os :: unix :: prelude :: * ;}
mkuse!{use core :: ops :: Deref ;}
mkuse!{use core :: ptr ;}
mkuse!{use core :: slice ;}
mkuse!{# [cfg (not (all (target_os = "linux" , target_env = "gnu")))] use libc :: mmap as mmap64 ;}
mkuse!{# [cfg (all (target_os = "linux" , target_env = "gnu"))] use libc :: mmap64 ;}
mkitem!{mkstruct!{pub struct Mmap { ptr : * mut libc :: c_void , len : usize , }}}
mkitem!{mkimpl!{impl Mmap { pub unsafe fn map (file : & File , len : usize , offset : u64) -> Option < Mmap > { let ptr = unsafe { mmap64 (ptr :: null_mut () , len , libc :: PROT_READ , libc :: MAP_PRIVATE , file . as_raw_fd () , offset . try_into () . ok () ? ,) } ; if ptr == libc :: MAP_FAILED { return None ; } Some (Mmap { ptr , len }) } }}}
mkitem!{mkimpl!{impl Deref for Mmap { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . ptr as * const u8 , self . len) } } }}}
mkitem!{mkimpl!{impl Drop for Mmap { fn drop (& mut self) { unsafe { let r = libc :: munmap (self . ptr , self . len) ; debug_assert_eq ! (r , 0) ; } } }}}
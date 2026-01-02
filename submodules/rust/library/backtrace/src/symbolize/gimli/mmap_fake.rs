mkuse!{use super :: File ;}
mkuse!{use super :: mystd :: io :: { Read , Seek , SeekFrom } ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: ops :: Deref ;}
mkitem!{mkstruct!{pub struct Mmap { vec : Vec < u8 > , }}}
mkitem!{mkimpl!{impl Mmap { pub unsafe fn map (mut file : & File , len : usize , offset : u64) -> Option < Mmap > { let mut mmap = Mmap { vec : Vec :: with_capacity (len) , } ; file . seek (SeekFrom :: Start (offset)) ; file . read_to_end (& mut mmap . vec) . ok () ? ; Some (mmap) } }}}
mkitem!{mkimpl!{impl Deref for Mmap { type Target = [u8] ; fn deref (& self) -> & [u8] { & self . vec [..] } }}}
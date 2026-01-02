mkuse!{use super :: super :: super :: windows_sys :: * ;}
mkuse!{use super :: mystd :: fs :: File ;}
mkuse!{use super :: mystd :: os :: windows :: prelude :: * ;}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: ops :: Deref ;}
mkuse!{use core :: ptr ;}
mkuse!{use core :: slice ;}
mkitem!{mkstruct!{pub struct Mmap { _file : File , ptr : * mut c_void , len : usize , }}}
mkitem!{mkimpl!{impl Mmap { pub unsafe fn map (file : & File , len : usize , offset : u64) -> Option < Mmap > { unsafe { let file = file . try_clone () . ok () ? ; let mapping = CreateFileMappingA (file . as_raw_handle () , ptr :: null_mut () , PAGE_READONLY , 0 , 0 , ptr :: null () ,) ; if mapping . is_null () { return None ; } let ptr = MapViewOfFile (mapping , FILE_MAP_READ , (offset >> 32) as u32 , offset as u32 , len ,) ; CloseHandle (mapping) ; if ptr . Value . is_null () { return None ; } Some (Mmap { _file : file , ptr : ptr . Value , len , }) } } }}}
mkitem!{mkimpl!{impl Deref for Mmap { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self . ptr . cast_const () . cast :: < u8 > () , self . len) } } }}}
mkitem!{mkimpl!{impl Drop for Mmap { fn drop (& mut self) { unsafe { let r = UnmapViewOfFile (MEMORY_MAPPED_VIEW_ADDRESS { Value : self . ptr }) ; debug_assert ! (r != 0) ; } } }}}
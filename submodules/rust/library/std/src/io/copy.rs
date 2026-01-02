mkuse!{use super :: { BorrowedBuf , BufReader , BufWriter , DEFAULT_BUF_SIZE , Read , Result , Write } ;}
mkuse!{use crate :: alloc :: Allocator ;}
mkuse!{use crate :: cmp ;}
mkuse!{use crate :: collections :: VecDeque ;}
mkuse!{use crate :: io :: IoSlice ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    # [doc = " Copies the entire contents of a reader into a writer."] # [doc = ""] # [doc = " This function will continuously read data from `reader` and then"] # [doc = " write it into `writer` in a streaming fashion until `reader`"] # [doc = " returns EOF."] # [doc = ""] # [doc = " On success, the total number of bytes that were copied from"] # [doc = " `reader` to `writer` is returned."] # [doc = ""] # [doc = " If you want to copy the contents of one file to another and you’re"] # [doc = " working with filesystem paths, see the [`fs::copy`] function."] # [doc = ""] # [doc = " [`fs::copy`]: crate::fs::copy"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error immediately if any call to [`read`] or"] # [doc = " [`write`] returns an error. All instances of [`ErrorKind::Interrupted`] are"] # [doc = " handled by this function and the underlying operation is retried."] # [doc = ""] # [doc = " [`read`]: Read::read"] # [doc = " [`write`]: Write::write"] # [doc = " [`ErrorKind::Interrupted`]: crate::io::ErrorKind::Interrupted"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io;"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let mut reader: &[u8] = b\"hello\";"] # [doc = "     let mut writer: Vec<u8> = vec![];"] # [doc = ""] # [doc = "     io::copy(&mut reader, &mut writer)?;"] # [doc = ""] # [doc = "     assert_eq!(&b\"hello\"[..], &writer[..]);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " On Linux (including Android), this function uses `copy_file_range(2)`,"] # [doc = " `sendfile(2)` or `splice(2)` syscalls to move data directly between file"] # [doc = " descriptors if possible."] # [doc = ""] # [doc = " Note that platform-specific behavior [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: crate::io#platform-specific-behavior"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn copy < R : ? Sized , W : ? Sized > (reader : & mut R , writer : & mut W) -> Result < u64 > where R : Read , W : Write , { cfg_select ! { any (target_os = "linux" , target_os = "android") => { crate :: sys :: kernel_copy :: copy_spec (reader , writer) } _ => { generic_copy (reader , writer) } } }
}

macro_rules! generic_copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generic_copy in module {}", module_path!());
    };
}

mkfn!{
    generic_copy_introspect!();
    # [doc = " The userspace read-write-loop implementation of `io::copy` that is used when"] # [doc = " OS-specific specializations for copy offloading are not available or not applicable."] pub (crate) fn generic_copy < R : ? Sized , W : ? Sized > (reader : & mut R , writer : & mut W) -> Result < u64 > where R : Read , W : Write , { let read_buf = BufferedReaderSpec :: buffer_size (reader) ; let write_buf = BufferedWriterSpec :: buffer_size (writer) ; if read_buf >= DEFAULT_BUF_SIZE && read_buf >= write_buf { return BufferedReaderSpec :: copy_to (reader , writer) ; } BufferedWriterSpec :: copy_from (writer , reader) }
}
mkitem!{mktrait!{# [doc = " Specialization of the read-write loop that reuses the internal"] # [doc = " buffer of a BufReader. If there's no buffer then the writer side"] # [doc = " should be used instead."] trait BufferedReaderSpec { fn buffer_size (& self) -> usize ; fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > ; }}}
mkitem!{mkimpl!{impl < T > BufferedReaderSpec for T where Self : Read , T : ? Sized , { # [inline] default fn buffer_size (& self) -> usize { 0 } default fn copy_to (& mut self , _to : & mut (impl Write + ? Sized)) -> Result < u64 > { unreachable ! ("only called from specializations") } }}}
mkitem!{mkimpl!{impl BufferedReaderSpec for & [u8] { fn buffer_size (& self) -> usize { usize :: MAX } fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > { let len = self . len () ; to . write_all (self) ? ; * self = & self [len ..] ; Ok (len as u64) } }}}
mkitem!{mkimpl!{impl < A : Allocator > BufferedReaderSpec for VecDeque < u8 , A > { fn buffer_size (& self) -> usize { usize :: MAX } fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > { let len = self . len () ; let (front , back) = self . as_slices () ; let bufs = & mut [IoSlice :: new (front) , IoSlice :: new (back)] ; to . write_all_vectored (bufs) ? ; self . clear () ; Ok (len as u64) } }}}
mkitem!{mkimpl!{impl < I > BufferedReaderSpec for BufReader < I > where Self : Read , I : ? Sized , { fn buffer_size (& self) -> usize { self . capacity () } fn copy_to (& mut self , to : & mut (impl Write + ? Sized)) -> Result < u64 > { let mut len = 0 ; loop { match self . read (& mut []) { Ok (_) => { } Err (e) if e . is_interrupted () => continue , Err (e) => return Err (e) , } let buf = self . buffer () ; if self . buffer () . len () == 0 { return Ok (len) ; } to . write_all (buf) ? ; len += buf . len () as u64 ; self . discard_buffer () ; } } }}}
mkitem!{mktrait!{# [doc = " Specialization of the read-write loop that either uses a stack buffer"] # [doc = " or reuses the internal buffer of a BufWriter"] trait BufferedWriterSpec : Write { fn buffer_size (& self) -> usize ; fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > ; }}}
mkitem!{mkimpl!{impl < W : Write + ? Sized > BufferedWriterSpec for W { # [inline] default fn buffer_size (& self) -> usize { 0 } default fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > { stack_buffer_copy (reader , self) } }}}
mkitem!{mkimpl!{impl < I : Write + ? Sized > BufferedWriterSpec for BufWriter < I > { fn buffer_size (& self) -> usize { self . capacity () } fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > { if self . capacity () < DEFAULT_BUF_SIZE { return stack_buffer_copy (reader , self) ; } let mut len = 0 ; let mut init = 0 ; loop { let buf = self . buffer_mut () ; let mut read_buf : BorrowedBuf < '_ > = buf . spare_capacity_mut () . into () ; unsafe { read_buf . set_init (init) ; } if read_buf . capacity () >= DEFAULT_BUF_SIZE { let mut cursor = read_buf . unfilled () ; match reader . read_buf (cursor . reborrow ()) { Ok (()) => { let bytes_read = cursor . written () ; if bytes_read == 0 { return Ok (len) ; } init = read_buf . init_len () - bytes_read ; len += bytes_read as u64 ; unsafe { buf . set_len (buf . len () + bytes_read) } ; } Err (ref e) if e . is_interrupted () => { } Err (e) => return Err (e) , } } else { init += buf . len () ; self . flush_buf () ? ; } } } }}}
mkitem!{mkimpl!{impl BufferedWriterSpec for Vec < u8 > { fn buffer_size (& self) -> usize { cmp :: max (DEFAULT_BUF_SIZE , self . capacity () - self . len ()) } fn copy_from < R : Read + ? Sized > (& mut self , reader : & mut R) -> Result < u64 > { reader . read_to_end (self) . map (| bytes | u64 :: try_from (bytes) . expect ("usize overflowed u64")) } }}}

macro_rules! stack_buffer_copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stack_buffer_copy in module {}", module_path!());
    };
}

mkfn!{
    stack_buffer_copy_introspect!();
    pub fn stack_buffer_copy < R : Read + ? Sized , W : Write + ? Sized > (reader : & mut R , writer : & mut W ,) -> Result < u64 > { let buf : & mut [_] = & mut [MaybeUninit :: uninit () ; DEFAULT_BUF_SIZE] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; let mut len = 0 ; loop { match reader . read_buf (buf . unfilled ()) { Ok (()) => { } Err (e) if e . is_interrupted () => continue , Err (e) => return Err (e) , } ; if buf . filled () . is_empty () { break ; } len += buf . filled () . len () as u64 ; writer . write_all (buf . filled ()) ? ; buf . clear () ; } Ok (len) }
}
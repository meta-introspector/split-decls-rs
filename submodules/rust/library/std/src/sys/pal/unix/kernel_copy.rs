mkuse!{# [cfg (not (any (all (target_os = "linux" , target_env = "gnu") , target_os = "hurd")))] use libc :: sendfile as sendfile64 ;}
mkuse!{# [cfg (any (all (target_os = "linux" , target_env = "gnu") , target_os = "hurd"))] use libc :: sendfile64 ;}
mkuse!{use libc :: { EBADF , EINVAL , ENOSYS , EOPNOTSUPP , EOVERFLOW , EPERM , EXDEV } ;}
mkuse!{use crate :: cmp :: min ;}
mkuse!{use crate :: fs :: { File , Metadata } ;}
mkuse!{use crate :: io :: copy :: generic_copy ;}
mkuse!{use crate :: io :: { BufRead , BufReader , BufWriter , Error , PipeReader , PipeWriter , Read , Result , StderrLock , StdinLock , StdoutLock , Take , Write , } ;}
mkuse!{use crate :: mem :: ManuallyDrop ;}
mkuse!{use crate :: net :: TcpStream ;}
mkuse!{use crate :: os :: unix :: fs :: FileTypeExt ;}
mkuse!{use crate :: os :: unix :: io :: { AsRawFd , FromRawFd , RawFd } ;}
mkuse!{use crate :: os :: unix :: net :: UnixStream ;}
mkuse!{use crate :: process :: { ChildStderr , ChildStdin , ChildStdout } ;}
mkuse!{use crate :: ptr ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicU8 , Ordering } ;}
mkuse!{use crate :: sys :: cvt ;}
mkuse!{use crate :: sys :: fs :: CachedFileMetadata ;}
mkuse!{use crate :: sys :: weak :: syscall ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! copy_spec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_spec in module {}", module_path!());
    };
}

mkfn!{
    copy_spec_introspect!();
    pub (crate) fn copy_spec < R : Read + ? Sized , W : Write + ? Sized > (read : & mut R , write : & mut W ,) -> Result < u64 > { let copier = Copier { read , write } ; SpecCopy :: copy (copier) }
}
mkitem!{mkenum!{# [doc = " This type represents either the inferred `FileType` of a `RawFd` based on the source"] # [doc = " type from which it was extracted or the actual metadata"] # [doc = ""] # [doc = " The methods on this type only provide hints, due to `AsRawFd` and `FromRawFd` the inferred"] # [doc = " type may be wrong."] enum FdMeta { Metadata (Metadata) , Socket , Pipe , # [doc = " We don't have any metadata because the stat syscall failed"] NoneObtained , }}}
mkitem!{mkenum!{# [derive (PartialEq)] enum FdHandle { Input , Output , }}}
mkitem!{mkimpl!{impl FdMeta { fn maybe_fifo (& self) -> bool { match self { FdMeta :: Metadata (meta) => meta . file_type () . is_fifo () , FdMeta :: Socket => false , FdMeta :: Pipe => true , FdMeta :: NoneObtained => true , } } fn potential_sendfile_source (& self) -> bool { match self { FdMeta :: Metadata (meta) if meta . file_type () . is_file () && meta . len () > 0 || meta . file_type () . is_block_device () => { true } _ => false , } } fn copy_file_range_candidate (& self , f : FdHandle) -> bool { match self { FdMeta :: Metadata (meta) if f == FdHandle :: Input && meta . is_file () && meta . len () > 0 => { true } FdMeta :: Metadata (meta) if f == FdHandle :: Output && meta . is_file () => true , _ => false , } } }}}

macro_rules! safe_kernel_copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function safe_kernel_copy in module {}", module_path!());
    };
}

mkfn!{
    safe_kernel_copy_introspect!();
    # [doc = " Returns true either if changes made to the source after a sendfile/splice call won't become"] # [doc = " visible in the sink or the source has explicitly opted into such behavior (e.g. by splicing"] # [doc = " a file into a pipe, the pipe being the source in this case)."] # [doc = ""] # [doc = " This will prevent File -> Pipe and File -> Socket splicing/sendfile optimizations to uphold"] # [doc = " the Read/Write API semantics of io::copy."] # [doc = ""] # [doc = " Note: This is not 100% airtight, the caller can use the RawFd conversion methods to turn a"] # [doc = " regular file into a TcpSocket which will be treated as a socket here without checking."] fn safe_kernel_copy (source : & FdMeta , sink : & FdMeta) -> bool { match (source , sink) { (FdMeta :: Socket , _) => true , (FdMeta :: Pipe , _) => true , (FdMeta :: Metadata (meta) , _) if meta . file_type () . is_fifo () || meta . file_type () . is_socket () => { true } (_ , FdMeta :: Metadata (meta)) if ! meta . file_type () . is_fifo () && ! meta . file_type () . is_socket () => { true } _ => false , } }
}
mkitem!{mkstruct!{struct CopyParams (FdMeta , Option < RawFd >) ;}}
mkitem!{mkstruct!{struct Copier < 'a , 'b , R : Read + ? Sized , W : Write + ? Sized > { read : & 'a mut R , write : & 'b mut W , }}}
mkitem!{mktrait!{trait SpecCopy { fn copy (self) -> Result < u64 > ; }}}
mkitem!{mkimpl!{impl < R : Read + ? Sized , W : Write + ? Sized > SpecCopy for Copier < '_ , '_ , R , W > { default fn copy (self) -> Result < u64 > { generic_copy (self . read , self . write) } }}}
mkitem!{mkimpl!{impl < R : CopyRead , W : CopyWrite > SpecCopy for Copier < '_ , '_ , R , W > { fn copy (self) -> Result < u64 > { let (reader , writer) = (self . read , self . write) ; let r_cfg = reader . properties () ; let w_cfg = writer . properties () ; let mut flush = | | -> Result < u64 > { let bytes = reader . drain_to (writer , u64 :: MAX) ? ; writer . flush () ? ; Ok (bytes) } ; let mut written = 0u64 ; if let (CopyParams (input_meta , Some (readfd)) , CopyParams (output_meta , Some (writefd))) = (r_cfg , w_cfg) { written += flush () ? ; let max_write = reader . min_limit () ; if input_meta . copy_file_range_candidate (FdHandle :: Input) && output_meta . copy_file_range_candidate (FdHandle :: Output) { let result = copy_regular_files (readfd , writefd , max_write) ; result . update_take (reader) ; match result { CopyResult :: Ended (bytes_copied) => return Ok (bytes_copied + written) , CopyResult :: Error (e , _) => return Err (e) , CopyResult :: Fallback (bytes) => written += bytes , } } if input_meta . potential_sendfile_source () && safe_kernel_copy (& input_meta , & output_meta) { let result = sendfile_splice (SpliceMode :: Sendfile , readfd , writefd , max_write) ; result . update_take (reader) ; match result { CopyResult :: Ended (bytes_copied) => return Ok (bytes_copied + written) , CopyResult :: Error (e , _) => return Err (e) , CopyResult :: Fallback (bytes) => written += bytes , } } if (input_meta . maybe_fifo () || output_meta . maybe_fifo ()) && safe_kernel_copy (& input_meta , & output_meta) { let result = sendfile_splice (SpliceMode :: Splice , readfd , writefd , max_write) ; result . update_take (reader) ; match result { CopyResult :: Ended (bytes_copied) => return Ok (bytes_copied + written) , CopyResult :: Error (e , _) => return Err (e) , CopyResult :: Fallback (0) => { } CopyResult :: Fallback (_) => { unreachable ! ("splice should not return > 0 bytes on the fallback path") } } } } match generic_copy (reader , writer) { Ok (bytes) => Ok (bytes + written) , err => err , } } }}}
mkitem!{mktrait!{# [rustc_specialization_trait] trait CopyRead : Read { # [doc = " Implementations that contain buffers (i.e. `BufReader`) must transfer data from their internal"] # [doc = " buffers into `writer` until either the buffers are emptied or `limit` bytes have been"] # [doc = " transferred, whichever occurs sooner."] # [doc = " If nested buffers are present the outer buffers must be drained first."] # [doc = ""] # [doc = " This is necessary to directly bypass the wrapper types while preserving the data order"] # [doc = " when operating directly on the underlying file descriptors."] fn drain_to < W : Write > (& mut self , _writer : & mut W , _limit : u64) -> Result < u64 > { Ok (0) } # [doc = " Updates `Take` wrappers to remove the number of bytes copied."] fn taken (& mut self , _bytes : u64) { } # [doc = " The minimum of the limit of all `Take<_>` wrappers, `u64::MAX` otherwise."] # [doc = " This method does not account for data `BufReader` buffers and would underreport"] # [doc = " the limit of a `Take<BufReader<Take<_>>>` type. Thus its result is only valid"] # [doc = " after draining the buffers via `drain_to`."] fn min_limit (& self) -> u64 { u64 :: MAX } # [doc = " Extracts the file descriptor and hints/metadata, delegating through wrappers if necessary."] fn properties (& self) -> CopyParams ; }}}
mkitem!{mktrait!{# [rustc_specialization_trait] trait CopyWrite : Write { # [doc = " Extracts the file descriptor and hints/metadata, delegating through wrappers if necessary."] fn properties (& self) -> CopyParams ; }}}
mkitem!{mkimpl!{impl < T > CopyRead for & mut T where T : CopyRead , { fn drain_to < W : Write > (& mut self , writer : & mut W , limit : u64) -> Result < u64 > { (* * self) . drain_to (writer , limit) } fn taken (& mut self , bytes : u64) { (* * self) . taken (bytes) ; } fn min_limit (& self) -> u64 { (* * self) . min_limit () } fn properties (& self) -> CopyParams { (* * self) . properties () } }}}
mkitem!{mkimpl!{impl < T > CopyWrite for & mut T where T : CopyWrite , { fn properties (& self) -> CopyParams { (* * self) . properties () } }}}
mkitem!{mkimpl!{impl CopyRead for File { fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for & File { fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (* self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for File { fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for & File { fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (* self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for TcpStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for & TcpStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for TcpStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for & TcpStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for UnixStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for & UnixStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for UnixStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for & UnixStream { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Socket , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for PipeReader { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for & PipeReader { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for PipeWriter { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for & PipeWriter { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for ChildStdin { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for ChildStdout { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for ChildStderr { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Pipe , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyRead for StdinLock < '_ > { fn drain_to < W : Write > (& mut self , writer : & mut W , outer_limit : u64) -> Result < u64 > { let buf_reader = self . as_mut_buf () ; let buf = buf_reader . buffer () ; let buf = & buf [0 .. min (buf . len () , outer_limit . try_into () . unwrap_or (usize :: MAX))] ; let bytes_drained = buf . len () ; writer . write_all (buf) ? ; buf_reader . consume (bytes_drained) ; Ok (bytes_drained as u64) } fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for StdoutLock < '_ > { fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for StderrLock < '_ > { fn properties (& self) -> CopyParams { CopyParams (fd_to_meta (self) , Some (self . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl < T : CopyRead > CopyRead for Take < T > { fn drain_to < W : Write > (& mut self , writer : & mut W , outer_limit : u64) -> Result < u64 > { let local_limit = self . limit () ; let combined_limit = min (outer_limit , local_limit) ; let bytes_drained = self . get_mut () . drain_to (writer , combined_limit) ? ; self . set_limit (local_limit - bytes_drained) ; Ok (bytes_drained) } fn taken (& mut self , bytes : u64) { self . set_limit (self . limit () - bytes) ; self . get_mut () . taken (bytes) ; } fn min_limit (& self) -> u64 { min (Take :: limit (self) , self . get_ref () . min_limit ()) } fn properties (& self) -> CopyParams { self . get_ref () . properties () } }}}
mkitem!{mkimpl!{impl < T : ? Sized + CopyRead > CopyRead for BufReader < T > { fn drain_to < W : Write > (& mut self , writer : & mut W , outer_limit : u64) -> Result < u64 > { let buf = self . buffer () ; let buf = & buf [0 .. min (buf . len () , outer_limit . try_into () . unwrap_or (usize :: MAX))] ; let bytes = buf . len () ; writer . write_all (buf) ? ; self . consume (bytes) ; let remaining = outer_limit - bytes as u64 ; let inner_bytes = self . get_mut () . drain_to (writer , remaining) ? ; Ok (bytes as u64 + inner_bytes) } fn taken (& mut self , bytes : u64) { self . get_mut () . taken (bytes) ; } fn min_limit (& self) -> u64 { self . get_ref () . min_limit () } fn properties (& self) -> CopyParams { self . get_ref () . properties () } }}}
mkitem!{mkimpl!{impl < T : ? Sized + CopyWrite > CopyWrite for BufWriter < T > { fn properties (& self) -> CopyParams { self . get_ref () . properties () } }}}
mkitem!{mkimpl!{impl CopyRead for CachedFileMetadata { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Metadata (self . 1 . clone ()) , Some (self . 0 . as_raw_fd ())) } }}}
mkitem!{mkimpl!{impl CopyWrite for CachedFileMetadata { fn properties (& self) -> CopyParams { CopyParams (FdMeta :: Metadata (self . 1 . clone ()) , Some (self . 0 . as_raw_fd ())) } }}}

macro_rules! fd_to_meta_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fd_to_meta in module {}", module_path!());
    };
}

mkfn!{
    fd_to_meta_introspect!();
    fn fd_to_meta < T : AsRawFd > (fd : & T) -> FdMeta { let fd = fd . as_raw_fd () ; let file : ManuallyDrop < File > = ManuallyDrop :: new (unsafe { File :: from_raw_fd (fd) }) ; match file . metadata () { Ok (meta) => FdMeta :: Metadata (meta) , Err (_) => FdMeta :: NoneObtained , } }
}
mkitem!{mkenum!{pub (super) enum CopyResult { Ended (u64) , Error (Error , u64) , Fallback (u64) , }}}
mkitem!{mkimpl!{impl CopyResult { fn update_take (& self , reader : & mut impl CopyRead) { match * self { CopyResult :: Fallback (bytes) | CopyResult :: Ended (bytes) | CopyResult :: Error (_ , bytes) => reader . taken (bytes) , } } }}}
mkitem!{# [doc = " Invalid file descriptor."] # [doc = ""] # [doc = " Valid file descriptors are guaranteed to be positive numbers (see `open()` manpage)"] # [doc = " while negative values are used to indicate errors."] # [doc = " Thus -1 will never be overlap with a valid open file."] const INVALID_FD : RawFd = - 1 ;}

macro_rules! copy_regular_files_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy_regular_files in module {}", module_path!());
    };
}

mkfn!{
    copy_regular_files_introspect!();
    # [doc = " Linux-specific implementation that will attempt to use copy_file_range for copy offloading."] # [doc = " As the name says, it only works on regular files."] # [doc = ""] # [doc = " Callers must handle fallback to a generic copy loop."] # [doc = " `Fallback` may indicate non-zero number of bytes already written"] # [doc = " if one of the files' cursor +`max_len` would exceed u64::MAX (`EOVERFLOW`)."] pub (super) fn copy_regular_files (reader : RawFd , writer : RawFd , max_len : u64) -> CopyResult { use crate :: cmp ; const NOT_PROBED : u8 = 0 ; const UNAVAILABLE : u8 = 1 ; const AVAILABLE : u8 = 2 ; static HAS_COPY_FILE_RANGE : Atomic < u8 > = AtomicU8 :: new (NOT_PROBED) ; let mut have_probed = match HAS_COPY_FILE_RANGE . load (Ordering :: Relaxed) { NOT_PROBED => false , UNAVAILABLE => return CopyResult :: Fallback (0) , _ => true , } ; syscall ! (fn copy_file_range (fd_in : libc :: c_int , off_in : * mut libc :: loff_t , fd_out : libc :: c_int , off_out : * mut libc :: loff_t , len : libc :: size_t , flags : libc :: c_uint ,) -> libc :: ssize_t ;) ; fn probe_copy_file_range_support () -> u8 { match unsafe { cvt (copy_file_range (INVALID_FD , ptr :: null_mut () , INVALID_FD , ptr :: null_mut () , 1 , 0)) . map_err (| e | e . raw_os_error ()) } { Err (Some (EPERM | ENOSYS)) => UNAVAILABLE , Err (Some (EBADF)) => AVAILABLE , Ok (_) => panic ! ("unexpected copy_file_range probe success") , Err (_) => UNAVAILABLE , } } let mut written = 0u64 ; while written < max_len { let bytes_to_copy = cmp :: min (max_len - written , usize :: MAX as u64) ; let bytes_to_copy = cmp :: min (bytes_to_copy as usize , 0x4000_0000usize) ; let copy_result = unsafe { cvt (copy_file_range (reader , ptr :: null_mut () , writer , ptr :: null_mut () , bytes_to_copy , 0)) } ; if ! have_probed && copy_result . is_ok () { have_probed = true ; HAS_COPY_FILE_RANGE . store (AVAILABLE , Ordering :: Relaxed) ; } match copy_result { Ok (0) if written == 0 => { return CopyResult :: Fallback (0) ; } Ok (0) => return CopyResult :: Ended (written) , Ok (ret) => written += ret as u64 , Err (err) => { return match err . raw_os_error () { Some (EOVERFLOW) => CopyResult :: Fallback (written) , Some (raw_os_error @ (ENOSYS | EXDEV | EINVAL | EPERM | EOPNOTSUPP | EBADF)) if written == 0 => { if ! have_probed { let available = if matches ! (raw_os_error , ENOSYS | EOPNOTSUPP | EPERM) { probe_copy_file_range_support () } else { AVAILABLE } ; HAS_COPY_FILE_RANGE . store (available , Ordering :: Relaxed) ; } CopyResult :: Fallback (0) } _ => CopyResult :: Error (err , written) , } ; } } } CopyResult :: Ended (written) }
}
mkitem!{mkenum!{# [derive (PartialEq)] enum SpliceMode { Sendfile , Splice , }}}

macro_rules! sendfile_splice_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sendfile_splice in module {}", module_path!());
    };
}

mkfn!{
    sendfile_splice_introspect!();
    # [doc = " performs splice or sendfile between file descriptors"] # [doc = " Does _not_ fall back to a generic copy loop."] fn sendfile_splice (mode : SpliceMode , reader : RawFd , writer : RawFd , len : u64) -> CopyResult { static HAS_SENDFILE : Atomic < bool > = AtomicBool :: new (true) ; static HAS_SPLICE : Atomic < bool > = AtomicBool :: new (true) ; # [cfg (target_os = "android")] syscall ! (fn splice (srcfd : libc :: c_int , src_offset : * const i64 , dstfd : libc :: c_int , dst_offset : * const i64 , len : libc :: size_t , flags : libc :: c_int ,) -> libc :: ssize_t ;) ; # [cfg (target_os = "linux")] use libc :: splice ; match mode { SpliceMode :: Sendfile if ! HAS_SENDFILE . load (Ordering :: Relaxed) => { return CopyResult :: Fallback (0) ; } SpliceMode :: Splice if ! HAS_SPLICE . load (Ordering :: Relaxed) => { return CopyResult :: Fallback (0) ; } _ => () , } let mut written = 0u64 ; while written < len { let chunk_size = crate :: cmp :: min (len - written , 0x7ffff000_u64) as usize ; let result = match mode { SpliceMode :: Sendfile => { cvt (unsafe { sendfile64 (writer , reader , ptr :: null_mut () , chunk_size) }) } SpliceMode :: Splice => cvt (unsafe { splice (reader , ptr :: null_mut () , writer , ptr :: null_mut () , chunk_size , 0) }) , } ; match result { Ok (0) => break , Ok (ret) => written += ret as u64 , Err (err) => { return match err . raw_os_error () { Some (ENOSYS | EPERM) => { match mode { SpliceMode :: Sendfile => HAS_SENDFILE . store (false , Ordering :: Relaxed) , SpliceMode :: Splice => HAS_SPLICE . store (false , Ordering :: Relaxed) , } assert_eq ! (written , 0) ; CopyResult :: Fallback (0) } Some (EINVAL) => { assert_eq ! (written , 0) ; CopyResult :: Fallback (0) } Some (os_err) if mode == SpliceMode :: Sendfile && os_err == EOVERFLOW => { CopyResult :: Fallback (written) } _ => CopyResult :: Error (err , written) , } ; } } } CopyResult :: Ended (written) }
}
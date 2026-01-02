mkuse!{use crate :: alloc :: { Layout , alloc , dealloc } ;}
mkuse!{use crate :: borrow :: Cow ;}
mkuse!{use crate :: ffi :: { OsStr , OsString , c_void } ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , Error , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: mem :: { self , MaybeUninit , offset_of } ;}
mkuse!{use crate :: os :: windows :: io :: { AsHandle , BorrowedHandle } ;}
mkuse!{use crate :: os :: windows :: prelude :: * ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: handle :: Handle ;}
mkuse!{use crate :: sys :: pal :: api :: { self , WinError , set_file_information_by_handle } ;}
mkuse!{use crate :: sys :: pal :: { IoResult , fill_utf16_buf , to_u16s , truncate_utf16_at_nul } ;}
mkuse!{use crate :: sys :: path :: { WCStr , maybe_verbatim } ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{use crate :: sys :: { Align8 , c , cvt } ;}
mkuse!{use crate :: sys_common :: { AsInner , FromInner , IntoInner } ;}
mkuse!{use crate :: { fmt , ptr , slice } ;}
mkmod!{remove_dir_all, { 
                getname!(remove_dir_all);
                getsrc!(remove_dir_all);
                getpath!(remove_dir_all);
                get_deps!(remove_dir_all);
                get_crates!(remove_dir_all);
                mkinclude!(remove_dir_all);
                 
            }}
mkuse!{use remove_dir_all :: remove_dir_all_iterative ;}
mkitem!{mkstruct!{pub struct File { handle : Handle , }}}
mkitem!{mkstruct!{# [derive (Clone)] pub struct FileAttr { attributes : u32 , creation_time : c :: FILETIME , last_access_time : c :: FILETIME , last_write_time : c :: FILETIME , change_time : Option < c :: FILETIME > , file_size : u64 , reparse_tag : u32 , volume_serial_number : Option < u32 > , number_of_links : Option < u32 > , file_index : Option < u64 > , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] pub struct FileType { is_directory : bool , is_symlink : bool , }}}
mkitem!{mkstruct!{pub struct ReadDir { handle : Option < FindNextFileHandle > , root : Arc < PathBuf > , first : Option < c :: WIN32_FIND_DATAW > , }}}
mkitem!{mkstruct!{struct FindNextFileHandle (c :: HANDLE) ;}}
mkitem!{mkimpl!{unsafe impl Send for FindNextFileHandle { }}}
mkitem!{mkimpl!{unsafe impl Sync for FindNextFileHandle { }}}
mkitem!{mkstruct!{pub struct DirEntry { root : Arc < PathBuf > , data : c :: WIN32_FIND_DATAW , }}}
mkitem!{mkimpl!{unsafe impl Send for OpenOptions { }}}
mkitem!{mkimpl!{unsafe impl Sync for OpenOptions { }}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct OpenOptions { read : bool , write : bool , append : bool , truncate : bool , create : bool , create_new : bool , custom_flags : u32 , access_mode : Option < u32 > , attributes : u32 , share_mode : u32 , security_qos_flags : u32 , inherit_handle : bool , }}}
mkitem!{mkstruct!{# [derive (Clone , PartialEq , Eq , Debug)] pub struct FilePermissions { attrs : u32 , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { accessed : Option < c :: FILETIME > , modified : Option < c :: FILETIME > , created : Option < c :: FILETIME > , }}}
mkitem!{mkimpl!{impl fmt :: Debug for c :: FILETIME { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let time = ((self . dwHighDateTime as u64) << 32) | self . dwLowDateTime as u64 ; f . debug_tuple ("FILETIME") . field (& time) . finish () } }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DirBuilder ;}}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * self . root , f) } }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { let Some (handle) = self . handle . as_ref () else { return None ; } ; if let Some (first) = self . first . take () { if let Some (e) = DirEntry :: new (& self . root , & first) { return Some (Ok (e)) ; } } unsafe { let mut wfd = mem :: zeroed () ; loop { if c :: FindNextFileW (handle . 0 , & mut wfd) == 0 { self . handle = None ; match api :: get_last_error () { WinError :: NO_MORE_FILES => return None , WinError { code } => { return Some (Err (Error :: from_raw_os_error (code as i32))) ; } } } if let Some (e) = DirEntry :: new (& self . root , & wfd) { return Some (Ok (e)) ; } } } } }}}
mkitem!{mkimpl!{impl Drop for FindNextFileHandle { fn drop (& mut self) { let r = unsafe { c :: FindClose (self . 0) } ; debug_assert ! (r != 0) ; } }}}
mkitem!{mkimpl!{impl DirEntry { fn new (root : & Arc < PathBuf > , wfd : & c :: WIN32_FIND_DATAW) -> Option < DirEntry > { match & wfd . cFileName [0 .. 3] { & [46 , 0 , ..] | & [46 , 46 , 0 , ..] => return None , _ => { } } Some (DirEntry { root : root . clone () , data : * wfd }) } pub fn path (& self) -> PathBuf { self . root . join (self . file_name ()) } pub fn file_name (& self) -> OsString { let filename = truncate_utf16_at_nul (& self . data . cFileName) ; OsString :: from_wide (filename) } pub fn file_type (& self) -> io :: Result < FileType > { Ok (FileType :: new (self . data . dwFileAttributes , self . data . dwReserved0 ,)) } pub fn metadata (& self) -> io :: Result < FileAttr > { Ok (self . data . into ()) } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { OpenOptions { read : false , write : false , append : false , truncate : false , create : false , create_new : false , custom_flags : 0 , access_mode : None , share_mode : c :: FILE_SHARE_READ | c :: FILE_SHARE_WRITE | c :: FILE_SHARE_DELETE , attributes : 0 , security_qos_flags : 0 , inherit_handle : false , } } pub fn read (& mut self , read : bool) { self . read = read ; } pub fn write (& mut self , write : bool) { self . write = write ; } pub fn append (& mut self , append : bool) { self . append = append ; } pub fn truncate (& mut self , truncate : bool) { self . truncate = truncate ; } pub fn create (& mut self , create : bool) { self . create = create ; } pub fn create_new (& mut self , create_new : bool) { self . create_new = create_new ; } pub fn custom_flags (& mut self , flags : u32) { self . custom_flags = flags ; } pub fn access_mode (& mut self , access_mode : u32) { self . access_mode = Some (access_mode) ; } pub fn share_mode (& mut self , share_mode : u32) { self . share_mode = share_mode ; } pub fn attributes (& mut self , attrs : u32) { self . attributes = attrs ; } pub fn security_qos_flags (& mut self , flags : u32) { self . security_qos_flags = flags | c :: SECURITY_SQOS_PRESENT ; } pub fn inherit_handle (& mut self , inherit : bool) { self . inherit_handle = inherit ; } fn get_access_mode (& self) -> io :: Result < u32 > { match (self . read , self . write , self . append , self . access_mode) { (.. , Some (mode)) => Ok (mode) , (true , false , false , None) => Ok (c :: GENERIC_READ) , (false , true , false , None) => Ok (c :: GENERIC_WRITE) , (true , true , false , None) => Ok (c :: GENERIC_READ | c :: GENERIC_WRITE) , (false , _ , true , None) => Ok (c :: FILE_GENERIC_WRITE & ! c :: FILE_WRITE_DATA) , (true , _ , true , None) => { Ok (c :: GENERIC_READ | (c :: FILE_GENERIC_WRITE & ! c :: FILE_WRITE_DATA)) } (false , false , false , None) => { if self . create || self . create_new || self . truncate { Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "creating or truncating a file requires write or append access" ,)) } else { Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "must specify at least one of read, write, or append access" ,)) } } } } fn get_creation_mode (& self) -> io :: Result < u32 > { match (self . write , self . append) { (true , false) => { } (false , false) => { if self . truncate || self . create || self . create_new { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "creating or truncating a file requires write or append access" ,)) ; } } (_ , true) => { if self . truncate && ! self . create_new { return Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "creating or truncating a file requires write or append access" ,)) ; } } } Ok (match (self . create , self . truncate , self . create_new) { (false , false , false) => c :: OPEN_EXISTING , (true , false , false) => c :: OPEN_ALWAYS , (false , true , false) => c :: TRUNCATE_EXISTING , (true , true , false) => c :: OPEN_ALWAYS , (_ , _ , true) => c :: CREATE_NEW , }) } fn get_flags_and_attributes (& self) -> u32 { self . custom_flags | self . attributes | self . security_qos_flags | if self . create_new { c :: FILE_FLAG_OPEN_REPARSE_POINT } else { 0 } } }}}
mkitem!{mkimpl!{impl File { pub fn open (path : & Path , opts : & OpenOptions) -> io :: Result < File > { let path = maybe_verbatim (path) ? ; let path = unsafe { WCStr :: from_wchars_with_null_unchecked (& path) } ; Self :: open_native (& path , opts) } fn open_native (path : & WCStr , opts : & OpenOptions) -> io :: Result < File > { let creation = opts . get_creation_mode () ? ; let sa = c :: SECURITY_ATTRIBUTES { nLength : size_of :: < c :: SECURITY_ATTRIBUTES > () as u32 , lpSecurityDescriptor : ptr :: null_mut () , bInheritHandle : opts . inherit_handle as c :: BOOL , } ; let handle = unsafe { c :: CreateFileW (path . as_ptr () , opts . get_access_mode () ? , opts . share_mode , if opts . inherit_handle { & sa } else { ptr :: null () } , creation , opts . get_flags_and_attributes () , ptr :: null_mut () ,) } ; let handle = unsafe { HandleOrInvalid :: from_raw_handle (handle) } ; if let Ok (handle) = OwnedHandle :: try_from (handle) { if opts . truncate && creation == c :: OPEN_ALWAYS && api :: get_last_error () == WinError :: ALREADY_EXISTS { let alloc = c :: FILE_ALLOCATION_INFO { AllocationSize : 0 } ; set_file_information_by_handle (handle . as_raw_handle () , & alloc) . or_else (| _ | { let eof = c :: FILE_END_OF_FILE_INFO { EndOfFile : 0 } ; set_file_information_by_handle (handle . as_raw_handle () , & eof) }) . io_result () ? ; } Ok (File { handle : Handle :: from_inner (handle) }) } else { Err (Error :: last_os_error ()) } } pub fn fsync (& self) -> io :: Result < () > { cvt (unsafe { c :: FlushFileBuffers (self . handle . as_raw_handle ()) }) ? ; Ok (()) } pub fn datasync (& self) -> io :: Result < () > { self . fsync () } fn acquire_lock (& self , flags : c :: LOCK_FILE_FLAGS) -> io :: Result < () > { unsafe { let mut overlapped : c :: OVERLAPPED = mem :: zeroed () ; let event = c :: CreateEventW (ptr :: null_mut () , c :: FALSE , c :: FALSE , ptr :: null ()) ; if event . is_null () { return Err (io :: Error :: last_os_error ()) ; } overlapped . hEvent = event ; let lock_result = cvt (c :: LockFileEx (self . handle . as_raw_handle () , flags , 0 , u32 :: MAX , u32 :: MAX , & mut overlapped ,)) ; let final_result = match lock_result { Ok (_) => Ok (()) , Err (err) => { if err . raw_os_error () == Some (c :: ERROR_IO_PENDING as i32) { let mut bytes_transferred = 0 ; cvt (c :: GetOverlappedResult (self . handle . as_raw_handle () , & mut overlapped , & mut bytes_transferred , c :: TRUE ,)) . map (| _ | ()) } else { Err (err) } } } ; c :: CloseHandle (overlapped . hEvent) ; final_result } } pub fn lock (& self) -> io :: Result < () > { self . acquire_lock (c :: LOCKFILE_EXCLUSIVE_LOCK) } pub fn lock_shared (& self) -> io :: Result < () > { self . acquire_lock (0) } pub fn try_lock (& self) -> Result < () , TryLockError > { let result = cvt (unsafe { let mut overlapped = mem :: zeroed () ; c :: LockFileEx (self . handle . as_raw_handle () , c :: LOCKFILE_EXCLUSIVE_LOCK | c :: LOCKFILE_FAIL_IMMEDIATELY , 0 , u32 :: MAX , u32 :: MAX , & mut overlapped ,) }) ; match result { Ok (_) => Ok (()) , Err (err) if err . raw_os_error () == Some (c :: ERROR_LOCK_VIOLATION as i32) => { Err (TryLockError :: WouldBlock) } Err (err) => Err (TryLockError :: Error (err)) , } } pub fn try_lock_shared (& self) -> Result < () , TryLockError > { let result = cvt (unsafe { let mut overlapped = mem :: zeroed () ; c :: LockFileEx (self . handle . as_raw_handle () , c :: LOCKFILE_FAIL_IMMEDIATELY , 0 , u32 :: MAX , u32 :: MAX , & mut overlapped ,) }) ; match result { Ok (_) => Ok (()) , Err (err) if err . raw_os_error () == Some (c :: ERROR_LOCK_VIOLATION as i32) => { Err (TryLockError :: WouldBlock) } Err (err) => Err (TryLockError :: Error (err)) , } } pub fn unlock (& self) -> io :: Result < () > { cvt (unsafe { c :: UnlockFile (self . handle . as_raw_handle () , 0 , 0 , u32 :: MAX , u32 :: MAX) }) ? ; let result = cvt (unsafe { c :: UnlockFile (self . handle . as_raw_handle () , 0 , 0 , u32 :: MAX , u32 :: MAX) }) ; match result { Ok (_) => Ok (()) , Err (err) if err . raw_os_error () == Some (c :: ERROR_NOT_LOCKED as i32) => Ok (()) , Err (err) => Err (err) , } } pub fn truncate (& self , size : u64) -> io :: Result < () > { let info = c :: FILE_END_OF_FILE_INFO { EndOfFile : size as i64 } ; api :: set_file_information_by_handle (self . handle . as_raw_handle () , & info) . io_result () } # [cfg (not (target_vendor = "uwp"))] pub fn file_attr (& self) -> io :: Result < FileAttr > { unsafe { let mut info : c :: BY_HANDLE_FILE_INFORMATION = mem :: zeroed () ; cvt (c :: GetFileInformationByHandle (self . handle . as_raw_handle () , & mut info)) ? ; let mut reparse_tag = 0 ; if info . dwFileAttributes & c :: FILE_ATTRIBUTE_REPARSE_POINT != 0 { let mut attr_tag : c :: FILE_ATTRIBUTE_TAG_INFO = mem :: zeroed () ; cvt (c :: GetFileInformationByHandleEx (self . handle . as_raw_handle () , c :: FileAttributeTagInfo , (& raw mut attr_tag) . cast () , size_of :: < c :: FILE_ATTRIBUTE_TAG_INFO > () . try_into () . unwrap () ,)) ? ; if attr_tag . FileAttributes & c :: FILE_ATTRIBUTE_REPARSE_POINT != 0 { reparse_tag = attr_tag . ReparseTag ; } } Ok (FileAttr { attributes : info . dwFileAttributes , creation_time : info . ftCreationTime , last_access_time : info . ftLastAccessTime , last_write_time : info . ftLastWriteTime , change_time : None , file_size : (info . nFileSizeLow as u64) | ((info . nFileSizeHigh as u64) << 32) , reparse_tag , volume_serial_number : Some (info . dwVolumeSerialNumber) , number_of_links : Some (info . nNumberOfLinks) , file_index : Some ((info . nFileIndexLow as u64) | ((info . nFileIndexHigh as u64) << 32) ,) , }) } } # [cfg (target_vendor = "uwp")] pub fn file_attr (& self) -> io :: Result < FileAttr > { unsafe { let mut info : c :: FILE_BASIC_INFO = mem :: zeroed () ; let size = size_of_val (& info) ; cvt (c :: GetFileInformationByHandleEx (self . handle . as_raw_handle () , c :: FileBasicInfo , (& raw mut info) as * mut c_void , size as u32 ,)) ? ; let mut attr = FileAttr { attributes : info . FileAttributes , creation_time : c :: FILETIME { dwLowDateTime : info . CreationTime as u32 , dwHighDateTime : (info . CreationTime >> 32) as u32 , } , last_access_time : c :: FILETIME { dwLowDateTime : info . LastAccessTime as u32 , dwHighDateTime : (info . LastAccessTime >> 32) as u32 , } , last_write_time : c :: FILETIME { dwLowDateTime : info . LastWriteTime as u32 , dwHighDateTime : (info . LastWriteTime >> 32) as u32 , } , change_time : Some (c :: FILETIME { dwLowDateTime : info . ChangeTime as u32 , dwHighDateTime : (info . ChangeTime >> 32) as u32 , }) , file_size : 0 , reparse_tag : 0 , volume_serial_number : None , number_of_links : None , file_index : None , } ; let mut info : c :: FILE_STANDARD_INFO = mem :: zeroed () ; let size = size_of_val (& info) ; cvt (c :: GetFileInformationByHandleEx (self . handle . as_raw_handle () , c :: FileStandardInfo , (& raw mut info) as * mut c_void , size as u32 ,)) ? ; attr . file_size = info . AllocationSize as u64 ; attr . number_of_links = Some (info . NumberOfLinks) ; if attr . attributes & c :: FILE_ATTRIBUTE_REPARSE_POINT != 0 { let mut attr_tag : c :: FILE_ATTRIBUTE_TAG_INFO = mem :: zeroed () ; cvt (c :: GetFileInformationByHandleEx (self . handle . as_raw_handle () , c :: FileAttributeTagInfo , (& raw mut attr_tag) . cast () , size_of :: < c :: FILE_ATTRIBUTE_TAG_INFO > () . try_into () . unwrap () ,)) ? ; if attr_tag . FileAttributes & c :: FILE_ATTRIBUTE_REPARSE_POINT != 0 { attr . reparse_tag = attr_tag . ReparseTag ; } } Ok (attr) } } pub fn read (& self , buf : & mut [u8]) -> io :: Result < usize > { self . handle . read (buf) } pub fn read_vectored (& self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . handle . read_vectored (bufs) } # [inline] pub fn is_read_vectored (& self) -> bool { self . handle . is_read_vectored () } pub fn read_at (& self , buf : & mut [u8] , offset : u64) -> io :: Result < usize > { self . handle . read_at (buf , offset) } pub fn read_buf (& self , cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . handle . read_buf (cursor) } pub fn read_buf_at (& self , cursor : BorrowedCursor < '_ > , offset : u64) -> io :: Result < () > { self . handle . read_buf_at (cursor , offset) } pub fn write (& self , buf : & [u8]) -> io :: Result < usize > { self . handle . write (buf) } pub fn write_vectored (& self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . handle . write_vectored (bufs) } # [inline] pub fn is_write_vectored (& self) -> bool { self . handle . is_write_vectored () } pub fn write_at (& self , buf : & [u8] , offset : u64) -> io :: Result < usize > { self . handle . write_at (buf , offset) } pub fn flush (& self) -> io :: Result < () > { Ok (()) } pub fn seek (& self , pos : SeekFrom) -> io :: Result < u64 > { let (whence , pos) = match pos { SeekFrom :: Start (n) => (c :: FILE_BEGIN , n as i64) , SeekFrom :: End (n) => (c :: FILE_END , n) , SeekFrom :: Current (n) => (c :: FILE_CURRENT , n) , } ; let pos = pos as i64 ; let mut newpos = 0 ; cvt (unsafe { c :: SetFilePointerEx (self . handle . as_raw_handle () , pos , & mut newpos , whence) }) ? ; Ok (newpos as u64) } pub fn size (& self) -> Option < io :: Result < u64 > > { let mut result = 0 ; Some (cvt (unsafe { c :: GetFileSizeEx (self . handle . as_raw_handle () , & mut result) }) . map (| _ | result as u64) ,) } pub fn tell (& self) -> io :: Result < u64 > { self . seek (SeekFrom :: Current (0)) } pub fn duplicate (& self) -> io :: Result < File > { Ok (Self { handle : self . handle . try_clone () ? }) } fn reparse_point (& self , space : & mut Align8 < [MaybeUninit < u8 >] > ,) -> io :: Result < (u32 , * mut c :: REPARSE_DATA_BUFFER) > { unsafe { let mut bytes = 0 ; cvt ({ let len = space . 0 . len () ; c :: DeviceIoControl (self . handle . as_raw_handle () , c :: FSCTL_GET_REPARSE_POINT , ptr :: null_mut () , 0 , space . 0 . as_mut_ptr () . cast () , len as u32 , & mut bytes , ptr :: null_mut () ,) }) ? ; const _ : () = assert ! (align_of ::< c :: REPARSE_DATA_BUFFER > () <= 8) ; Ok ((bytes , space . 0 . as_mut_ptr () . cast :: < c :: REPARSE_DATA_BUFFER > ())) } } fn readlink (& self) -> io :: Result < PathBuf > { let mut space = Align8 ([MaybeUninit :: < u8 > :: uninit () ; c :: MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize]) ; let (_bytes , buf) = self . reparse_point (& mut space) ? ; unsafe { let (path_buffer , subst_off , subst_len , relative) = match (* buf) . ReparseTag { c :: IO_REPARSE_TAG_SYMLINK => { let info : * mut c :: SYMBOLIC_LINK_REPARSE_BUFFER = (& raw mut (* buf) . rest) . cast () ; assert ! (info . is_aligned ()) ; ((& raw mut (* info) . PathBuffer) . cast :: < u16 > () , (* info) . SubstituteNameOffset / 2 , (* info) . SubstituteNameLength / 2 , (* info) . Flags & c :: SYMLINK_FLAG_RELATIVE != 0 ,) } c :: IO_REPARSE_TAG_MOUNT_POINT => { let info : * mut c :: MOUNT_POINT_REPARSE_BUFFER = (& raw mut (* buf) . rest) . cast () ; assert ! (info . is_aligned ()) ; ((& raw mut (* info) . PathBuffer) . cast :: < u16 > () , (* info) . SubstituteNameOffset / 2 , (* info) . SubstituteNameLength / 2 , false ,) } _ => { return Err (io :: const_error ! (io :: ErrorKind :: Uncategorized , "Unsupported reparse point type" ,)) ; } } ; let subst_ptr = path_buffer . add (subst_off . into ()) ; let subst = slice :: from_raw_parts_mut (subst_ptr , subst_len as usize) ; if ! relative && subst . starts_with (& [92u16 , 63u16 , 63u16 , 92u16]) { subst [1] = b'\\' as u16 ; let user = crate :: sys :: args :: from_wide_to_user_path (subst . iter () . copied () . chain ([0]) . collect () ,) ? ; Ok (PathBuf :: from (OsString :: from_wide (user . strip_suffix (& [0]) . unwrap_or (& user)))) } else { Ok (PathBuf :: from (OsString :: from_wide (subst))) } } } pub fn set_permissions (& self , perm : FilePermissions) -> io :: Result < () > { let info = c :: FILE_BASIC_INFO { CreationTime : 0 , LastAccessTime : 0 , LastWriteTime : 0 , ChangeTime : 0 , FileAttributes : perm . attrs , } ; api :: set_file_information_by_handle (self . handle . as_raw_handle () , & info) . io_result () } pub fn set_times (& self , times : FileTimes) -> io :: Result < () > { let is_zero = | t : c :: FILETIME | t . dwLowDateTime == 0 && t . dwHighDateTime == 0 ; if times . accessed . map_or (false , is_zero) || times . modified . map_or (false , is_zero) || times . created . map_or (false , is_zero) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "cannot set file timestamp to 0" ,)) ; } let is_max = | t : c :: FILETIME | t . dwLowDateTime == u32 :: MAX && t . dwHighDateTime == u32 :: MAX ; if times . accessed . map_or (false , is_max) || times . modified . map_or (false , is_max) || times . created . map_or (false , is_max) { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "cannot set file timestamp to 0xFFFF_FFFF_FFFF_FFFF" ,)) ; } cvt (unsafe { let created = times . created . as_ref () . map (| a | a as * const c :: FILETIME) . unwrap_or (ptr :: null ()) ; let accessed = times . accessed . as_ref () . map (| a | a as * const c :: FILETIME) . unwrap_or (ptr :: null ()) ; let modified = times . modified . as_ref () . map (| a | a as * const c :: FILETIME) . unwrap_or (ptr :: null ()) ; c :: SetFileTime (self . as_raw_handle () , created , accessed , modified) }) ? ; Ok (()) } # [doc = " Gets only basic file information such as attributes and file times."] fn basic_info (& self) -> io :: Result < c :: FILE_BASIC_INFO > { unsafe { let mut info : c :: FILE_BASIC_INFO = mem :: zeroed () ; let size = size_of_val (& info) ; cvt (c :: GetFileInformationByHandleEx (self . handle . as_raw_handle () , c :: FileBasicInfo , (& raw mut info) as * mut c_void , size as u32 ,)) ? ; Ok (info) } } # [doc = " Deletes the file, consuming the file handle to ensure the delete occurs"] # [doc = " as immediately as possible."] # [doc = " This attempts to use `posix_delete` but falls back to `win32_delete`"] # [doc = " if that is not supported by the filesystem."] # [allow (unused)] fn delete (self) -> Result < () , WinError > { match self . posix_delete () { Err (WinError :: INVALID_PARAMETER) | Err (WinError :: NOT_SUPPORTED) | Err (WinError :: INVALID_FUNCTION) => self . win32_delete () , result => result , } } # [doc = " Delete using POSIX semantics."] # [doc = ""] # [doc = " Files will be deleted as soon as the handle is closed. This is supported"] # [doc = " for Windows 10 1607 (aka RS1) and later. However some filesystem"] # [doc = " drivers will not support it even then, e.g. FAT32."] # [doc = ""] # [doc = " If the operation is not supported for this filesystem or OS version"] # [doc = " then errors will be `ERROR_NOT_SUPPORTED` or `ERROR_INVALID_PARAMETER`."] # [allow (unused)] fn posix_delete (& self) -> Result < () , WinError > { let info = c :: FILE_DISPOSITION_INFO_EX { Flags : c :: FILE_DISPOSITION_FLAG_DELETE | c :: FILE_DISPOSITION_FLAG_POSIX_SEMANTICS | c :: FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE , } ; api :: set_file_information_by_handle (self . handle . as_raw_handle () , & info) } # [doc = " Delete a file using win32 semantics. The file won't actually be deleted"] # [doc = " until all file handles are closed. However, marking a file for deletion"] # [doc = " will prevent anyone from opening a new handle to the file."] # [allow (unused)] fn win32_delete (& self) -> Result < () , WinError > { let info = c :: FILE_DISPOSITION_INFO { DeleteFile : true } ; api :: set_file_information_by_handle (self . handle . as_raw_handle () , & info) } # [doc = " Fill the given buffer with as many directory entries as will fit."] # [doc = " This will remember its position and continue from the last call unless"] # [doc = " `restart` is set to `true`."] # [doc = ""] # [doc = " The returned bool indicates if there are more entries or not."] # [doc = " It is an error if `self` is not a directory."] # [doc = ""] # [doc = " # Symlinks and other reparse points"] # [doc = ""] # [doc = " On Windows a file is either a directory or a non-directory."] # [doc = " A symlink directory is simply an empty directory with some \"reparse\" metadata attached."] # [doc = " So if you open a link (not its target) and iterate the directory,"] # [doc = " you will always iterate an empty directory regardless of the target."] # [allow (unused)] fn fill_dir_buff (& self , buffer : & mut DirBuff , restart : bool) -> Result < bool , WinError > { let class = if restart { c :: FileIdBothDirectoryRestartInfo } else { c :: FileIdBothDirectoryInfo } ; unsafe { let result = c :: GetFileInformationByHandleEx (self . as_raw_handle () , class , buffer . as_mut_ptr () . cast () , buffer . capacity () as _ ,) ; if result == 0 { let err = api :: get_last_error () ; if err . code == c :: ERROR_NO_MORE_FILES { Ok (false) } else { Err (err) } } else { Ok (true) } } } }}}
mkitem!{mkstruct!{# [doc = " A buffer for holding directory entries."] struct DirBuff { buffer : Box < Align8 < [MaybeUninit < u8 > ; Self :: BUFFER_SIZE] > > , }}}
mkitem!{mkimpl!{impl DirBuff { const BUFFER_SIZE : usize = 1024 ; fn new () -> Self { Self { buffer : unsafe { Box :: new_uninit () . assume_init () } , } } fn capacity (& self) -> usize { self . buffer . 0 . len () } fn as_mut_ptr (& mut self) -> * mut u8 { self . buffer . 0 . as_mut_ptr () . cast () } # [doc = " Returns a `DirBuffIter`."] fn iter (& self) -> DirBuffIter < '_ > { DirBuffIter :: new (self) } }}}
mkitem!{mkimpl!{impl AsRef < [MaybeUninit < u8 >] > for DirBuff { fn as_ref (& self) -> & [MaybeUninit < u8 >] { & self . buffer . 0 } }}}
mkitem!{mkstruct!{# [doc = " An iterator over entries stored in a `DirBuff`."] # [doc = ""] # [doc = " Currently only returns file names (UTF-16 encoded)."] struct DirBuffIter < 'a > { buffer : Option < & 'a [MaybeUninit < u8 >] > , cursor : usize , }}}
mkitem!{mkimpl!{impl < 'a > DirBuffIter < 'a > { fn new (buffer : & 'a DirBuff) -> Self { Self { buffer : Some (buffer . as_ref ()) , cursor : 0 } } }}}
mkitem!{mkimpl!{impl < 'a > Iterator for DirBuffIter < 'a > { type Item = (Cow < 'a , [u16] > , bool) ; fn next (& mut self) -> Option < Self :: Item > { let buffer = & self . buffer ? [self . cursor ..] ; let (name , is_directory , next_entry) = unsafe { let info = buffer . as_ptr () . cast :: < c :: FILE_ID_BOTH_DIR_INFO > () ; let next_entry = (& raw const (* info) . NextEntryOffset) . read_unaligned () as usize ; let length = (& raw const (* info) . FileNameLength) . read_unaligned () as usize ; let attrs = (& raw const (* info) . FileAttributes) . read_unaligned () ; let name = from_maybe_unaligned ((& raw const (* info) . FileName) . cast :: < u16 > () , length / size_of :: < u16 > () ,) ; let is_directory = (attrs & c :: FILE_ATTRIBUTE_DIRECTORY) != 0 ; (name , is_directory , next_entry) } ; if next_entry == 0 { self . buffer = None } else { self . cursor += next_entry } const DOT : u16 = b'.' as u16 ; match & name [..] { [DOT] | [DOT , DOT] => self . next () , _ => Some ((name , is_directory)) , } } }}}

macro_rules! from_maybe_unaligned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_maybe_unaligned in module {}", module_path!());
    };
}

mkfn!{
    from_maybe_unaligned_introspect!();
    unsafe fn from_maybe_unaligned < 'a > (p : * const u16 , len : usize) -> Cow < 'a , [u16] > { unsafe { if p . is_aligned () { Cow :: Borrowed (crate :: slice :: from_raw_parts (p , len)) } else { Cow :: Owned ((0 .. len) . map (| i | p . add (i) . read_unaligned ()) . collect ()) } } }
}
mkitem!{mkimpl!{impl AsInner < Handle > for File { # [inline] fn as_inner (& self) -> & Handle { & self . handle } }}}
mkitem!{mkimpl!{impl IntoInner < Handle > for File { fn into_inner (self) -> Handle { self . handle } }}}
mkitem!{mkimpl!{impl FromInner < Handle > for File { fn from_inner (handle : Handle) -> File { File { handle } } }}}
mkitem!{mkimpl!{impl AsHandle for File { fn as_handle (& self) -> BorrowedHandle < '_ > { self . as_inner () . as_handle () } }}}
mkitem!{mkimpl!{impl AsRawHandle for File { fn as_raw_handle (& self) -> RawHandle { self . as_inner () . as_raw_handle () } }}}
mkitem!{mkimpl!{impl IntoRawHandle for File { fn into_raw_handle (self) -> RawHandle { self . into_inner () . into_raw_handle () } }}}
mkitem!{mkimpl!{impl FromRawHandle for File { unsafe fn from_raw_handle (raw_handle : RawHandle) -> Self { unsafe { Self { handle : FromInner :: from_inner (FromRawHandle :: from_raw_handle (raw_handle)) } } } }}}
mkitem!{mkimpl!{impl fmt :: Debug for File { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut b = f . debug_struct ("File") ; b . field ("handle" , & self . handle . as_raw_handle ()) ; if let Ok (path) = get_path (self) { b . field ("path" , & path) ; } b . finish () } }}}
mkitem!{mkimpl!{impl FileAttr { pub fn size (& self) -> u64 { self . file_size } pub fn perm (& self) -> FilePermissions { FilePermissions { attrs : self . attributes } } pub fn attrs (& self) -> u32 { self . attributes } pub fn file_type (& self) -> FileType { FileType :: new (self . attributes , self . reparse_tag) } pub fn modified (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from (self . last_write_time)) } pub fn accessed (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from (self . last_access_time)) } pub fn created (& self) -> io :: Result < SystemTime > { Ok (SystemTime :: from (self . creation_time)) } pub fn modified_u64 (& self) -> u64 { to_u64 (& self . last_write_time) } pub fn accessed_u64 (& self) -> u64 { to_u64 (& self . last_access_time) } pub fn created_u64 (& self) -> u64 { to_u64 (& self . creation_time) } pub fn changed_u64 (& self) -> Option < u64 > { self . change_time . as_ref () . map (| c | to_u64 (c)) } pub fn volume_serial_number (& self) -> Option < u32 > { self . volume_serial_number } pub fn number_of_links (& self) -> Option < u32 > { self . number_of_links } pub fn file_index (& self) -> Option < u64 > { self . file_index } }}}
mkitem!{mkimpl!{impl From < c :: WIN32_FIND_DATAW > for FileAttr { fn from (wfd : c :: WIN32_FIND_DATAW) -> Self { FileAttr { attributes : wfd . dwFileAttributes , creation_time : wfd . ftCreationTime , last_access_time : wfd . ftLastAccessTime , last_write_time : wfd . ftLastWriteTime , change_time : None , file_size : ((wfd . nFileSizeHigh as u64) << 32) | (wfd . nFileSizeLow as u64) , reparse_tag : if wfd . dwFileAttributes & c :: FILE_ATTRIBUTE_REPARSE_POINT != 0 { wfd . dwReserved0 } else { 0 } , volume_serial_number : None , number_of_links : None , file_index : None , } } }}}

macro_rules! to_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_u64 in module {}", module_path!());
    };
}

mkfn!{
    to_u64_introspect!();
    fn to_u64 (ft : & c :: FILETIME) -> u64 { (ft . dwLowDateTime as u64) | ((ft . dwHighDateTime as u64) << 32) }
}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { self . attrs & c :: FILE_ATTRIBUTE_READONLY != 0 } pub fn set_readonly (& mut self , readonly : bool) { if readonly { self . attrs |= c :: FILE_ATTRIBUTE_READONLY ; } else { self . attrs &= ! c :: FILE_ATTRIBUTE_READONLY ; } } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , t : SystemTime) { self . accessed = Some (t . into_inner ()) ; } pub fn set_modified (& mut self , t : SystemTime) { self . modified = Some (t . into_inner ()) ; } pub fn set_created (& mut self , t : SystemTime) { self . created = Some (t . into_inner ()) ; } }}}
mkitem!{mkimpl!{impl FileType { fn new (attributes : u32 , reparse_tag : u32) -> FileType { let is_directory = attributes & c :: FILE_ATTRIBUTE_DIRECTORY != 0 ; let is_symlink = { let is_reparse_point = attributes & c :: FILE_ATTRIBUTE_REPARSE_POINT != 0 ; let is_reparse_tag_name_surrogate = reparse_tag & 0x20000000 != 0 ; is_reparse_point && is_reparse_tag_name_surrogate } ; FileType { is_directory , is_symlink } } pub fn is_dir (& self) -> bool { ! self . is_symlink && self . is_directory } pub fn is_file (& self) -> bool { ! self . is_symlink && ! self . is_directory } pub fn is_symlink (& self) -> bool { self . is_symlink } pub fn is_symlink_dir (& self) -> bool { self . is_symlink && self . is_directory } pub fn is_symlink_file (& self) -> bool { self . is_symlink && ! self . is_directory } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder } pub fn mkdir (& self , p : & Path) -> io :: Result < () > { let p = maybe_verbatim (p) ? ; cvt (unsafe { c :: CreateDirectoryW (p . as_ptr () , ptr :: null_mut ()) }) ? ; Ok (()) } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (p : & Path) -> io :: Result < ReadDir > { if p . as_os_str () . is_empty () { return Err (io :: Error :: from_raw_os_error (c :: ERROR_PATH_NOT_FOUND as i32)) ; } let root = p . to_path_buf () ; let star = p . join ("*") ; let path = maybe_verbatim (& star) ? ; unsafe { let mut wfd : c :: WIN32_FIND_DATAW = mem :: zeroed () ; let find_handle = c :: FindFirstFileExW (path . as_ptr () , c :: FindExInfoBasic , & mut wfd as * mut _ as _ , c :: FindExSearchNameMatch , ptr :: null () , 0 ,) ; if find_handle != c :: INVALID_HANDLE_VALUE { Ok (ReadDir { handle : Some (FindNextFileHandle (find_handle)) , root : Arc :: new (root) , first : Some (wfd) , }) } else { let last_error = api :: get_last_error () ; if last_error == WinError :: FILE_NOT_FOUND { return Ok (ReadDir { handle : None , root : Arc :: new (root) , first : None }) ; } Err (Error :: from_raw_os_error (last_error . code as i32)) } } }
}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (path : & WCStr) -> io :: Result < () > { if unsafe { c :: DeleteFileW (path . as_ptr ()) } == 0 { let err = api :: get_last_error () ; if err == WinError :: ACCESS_DENIED { let mut opts = OpenOptions :: new () ; opts . access_mode (c :: DELETE) ; opts . custom_flags (c :: FILE_FLAG_OPEN_REPARSE_POINT) ; if let Ok (f) = File :: open_native (& path , & opts) { if f . posix_delete () . is_ok () { return Ok (()) ; } } } Err (io :: Error :: from_raw_os_error (err . code as i32)) } else { Ok (()) } }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (old : & WCStr , new : & WCStr) -> io :: Result < () > { if unsafe { c :: MoveFileExW (old . as_ptr () , new . as_ptr () , c :: MOVEFILE_REPLACE_EXISTING) } == 0 { let err = api :: get_last_error () ; if err == WinError :: ACCESS_DENIED { let mut opts = OpenOptions :: new () ; opts . access_mode (c :: DELETE) ; opts . custom_flags (c :: FILE_FLAG_OPEN_REPARSE_POINT | c :: FILE_FLAG_BACKUP_SEMANTICS) ; let Ok (f) = File :: open_native (& old , & opts) else { return Err (err) . io_result () } ; let Ok (new_len_without_nul_in_bytes) : Result < u32 , _ > = ((new . count_bytes () - 1) * 2) . try_into () else { return Err (err) . io_result () ; } ; let offset : u32 = offset_of ! (c :: FILE_RENAME_INFO , FileName) . try_into () . unwrap () ; let struct_size = offset + new_len_without_nul_in_bytes + 2 ; let layout = Layout :: from_size_align (struct_size as usize , align_of :: < c :: FILE_RENAME_INFO > ()) . unwrap () ; let file_rename_info ; unsafe { file_rename_info = alloc (layout) . cast :: < c :: FILE_RENAME_INFO > () ; if file_rename_info . is_null () { return Err (io :: ErrorKind :: OutOfMemory . into ()) ; } (& raw mut (* file_rename_info) . Anonymous) . write (c :: FILE_RENAME_INFO_0 { Flags : c :: FILE_RENAME_FLAG_REPLACE_IF_EXISTS | c :: FILE_RENAME_FLAG_POSIX_SEMANTICS , }) ; (& raw mut (* file_rename_info) . RootDirectory) . write (ptr :: null_mut ()) ; (& raw mut (* file_rename_info) . FileNameLength) . write (new_len_without_nul_in_bytes) ; new . as_ptr () . copy_to_nonoverlapping ((& raw mut (* file_rename_info) . FileName) . cast :: < u16 > () , new . count_bytes () ,) ; } let result = unsafe { c :: SetFileInformationByHandle (f . as_raw_handle () , c :: FileRenameInfoEx , file_rename_info . cast :: < c_void > () , struct_size ,) } ; unsafe { dealloc (file_rename_info . cast :: < u8 > () , layout) } ; if result == 0 { if api :: get_last_error () == WinError :: DIR_NOT_EMPTY { return Err (WinError :: DIR_NOT_EMPTY) . io_result () ; } else { return Err (err) . io_result () ; } } } else { return Err (err) . io_result () ; } } Ok (()) }
}

macro_rules! rmdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rmdir in module {}", module_path!());
    };
}

mkfn!{
    rmdir_introspect!();
    pub fn rmdir (p : & WCStr) -> io :: Result < () > { cvt (unsafe { c :: RemoveDirectoryW (p . as_ptr ()) }) ? ; Ok (()) }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (path : & WCStr) -> io :: Result < () > { let mut opts = OpenOptions :: new () ; opts . access_mode (c :: FILE_LIST_DIRECTORY) ; opts . custom_flags (c :: FILE_FLAG_BACKUP_SEMANTICS | c :: FILE_FLAG_OPEN_REPARSE_POINT) ; let file = File :: open_native (path , & opts) ? ; if (file . basic_info () ? . FileAttributes & c :: FILE_ATTRIBUTE_DIRECTORY) == 0 { return Err (io :: Error :: from_raw_os_error (c :: ERROR_DIRECTORY as _)) ; } remove_dir_all_iterative (file) . io_result () }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (path : & WCStr) -> io :: Result < PathBuf > { let mut opts = OpenOptions :: new () ; opts . access_mode (0) ; opts . custom_flags (c :: FILE_FLAG_OPEN_REPARSE_POINT | c :: FILE_FLAG_BACKUP_SEMANTICS) ; let file = File :: open_native (& path , & opts) ? ; file . readlink () }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (original : & Path , link : & Path) -> io :: Result < () > { symlink_inner (original , link , false) }
}

macro_rules! symlink_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink_inner in module {}", module_path!());
    };
}

mkfn!{
    symlink_inner_introspect!();
    pub fn symlink_inner (original : & Path , link : & Path , dir : bool) -> io :: Result < () > { let original = to_u16s (original) ? ; let link = maybe_verbatim (link) ? ; let flags = if dir { c :: SYMBOLIC_LINK_FLAG_DIRECTORY } else { 0 } ; let result = cvt (unsafe { c :: CreateSymbolicLinkW (link . as_ptr () , original . as_ptr () , flags | c :: SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE ,) as c :: BOOL }) ; if let Err (err) = result { if err . raw_os_error () == Some (c :: ERROR_INVALID_PARAMETER as i32) { cvt (unsafe { c :: CreateSymbolicLinkW (link . as_ptr () , original . as_ptr () , flags) as c :: BOOL }) ? ; } else { return Err (err) ; } } Ok (()) }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    # [cfg (not (target_vendor = "uwp"))] pub fn link (original : & WCStr , link : & WCStr) -> io :: Result < () > { cvt (unsafe { c :: CreateHardLinkW (link . as_ptr () , original . as_ptr () , ptr :: null_mut ()) }) ? ; Ok (()) }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    # [cfg (target_vendor = "uwp")] pub fn link (_original : & WCStr , _link : & WCStr) -> io :: Result < () > { return Err (io :: const_error ! (io :: ErrorKind :: Unsupported , "hard link are not supported on UWP")) ; }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (path : & WCStr) -> io :: Result < FileAttr > { match metadata (path , ReparsePoint :: Follow) { Err (err) if err . raw_os_error () == Some (c :: ERROR_CANT_ACCESS_FILE as i32) => { if let Ok (attrs) = lstat (path) { if ! attrs . file_type () . is_symlink () { return Ok (attrs) ; } } Err (err) } result => result , } }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (path : & WCStr) -> io :: Result < FileAttr > { metadata (path , ReparsePoint :: Open) }
}
mkitem!{mkenum!{# [repr (u32)] # [derive (Clone , Copy , PartialEq , Eq)] enum ReparsePoint { Follow = 0 , Open = c :: FILE_FLAG_OPEN_REPARSE_POINT , }}}
mkitem!{mkimpl!{impl ReparsePoint { fn as_flag (self) -> u32 { self as u32 } }}}

macro_rules! metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function metadata in module {}", module_path!());
    };
}

mkfn!{
    metadata_introspect!();
    fn metadata (path : & WCStr , reparse : ReparsePoint) -> io :: Result < FileAttr > { let mut opts = OpenOptions :: new () ; opts . access_mode (0) ; opts . custom_flags (c :: FILE_FLAG_BACKUP_SEMANTICS | reparse . as_flag ()) ; match File :: open_native (& path , & opts) { Ok (file) => file . file_attr () , Err (e) if [Some (c :: ERROR_SHARING_VIOLATION as _) , Some (c :: ERROR_ACCESS_DENIED as _)] . contains (& e . raw_os_error ()) => { unsafe { let mut wfd : c :: WIN32_FIND_DATAW = mem :: zeroed () ; let handle = c :: FindFirstFileExW (path . as_ptr () , c :: FindExInfoBasic , & mut wfd as * mut _ as _ , c :: FindExSearchNameMatch , ptr :: null () , 0 ,) ; if handle == c :: INVALID_HANDLE_VALUE { Err (e) } else { c :: FindClose (handle) ; let attrs = FileAttr :: from (wfd) ; if reparse == ReparsePoint :: Follow && attrs . file_type () . is_symlink () { Err (e) } else { Ok (attrs) } } } } Err (e) => Err (e) , } }
}

macro_rules! set_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_perm in module {}", module_path!());
    };
}

mkfn!{
    set_perm_introspect!();
    pub fn set_perm (p : & WCStr , perm : FilePermissions) -> io :: Result < () > { unsafe { cvt (c :: SetFileAttributesW (p . as_ptr () , perm . attrs)) ? ; Ok (()) } }
}

macro_rules! get_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_path in module {}", module_path!());
    };
}

mkfn!{
    get_path_introspect!();
    fn get_path (f : & File) -> io :: Result < PathBuf > { fill_utf16_buf (| buf , sz | unsafe { c :: GetFinalPathNameByHandleW (f . handle . as_raw_handle () , buf , sz , c :: VOLUME_NAME_DOS) } , | buf | PathBuf :: from (OsString :: from_wide (buf)) ,) }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (p : & WCStr) -> io :: Result < PathBuf > { let mut opts = OpenOptions :: new () ; opts . access_mode (0) ; opts . custom_flags (c :: FILE_FLAG_BACKUP_SEMANTICS) ; let f = File :: open_native (p , & opts) ? ; get_path (& f) }
}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    pub fn copy (from : & WCStr , to : & WCStr) -> io :: Result < u64 > { unsafe extern "system" fn callback (_TotalFileSize : i64 , _TotalBytesTransferred : i64 , _StreamSize : i64 , StreamBytesTransferred : i64 , dwStreamNumber : u32 , _dwCallbackReason : u32 , _hSourceFile : c :: HANDLE , _hDestinationFile : c :: HANDLE , lpData : * const c_void ,) -> u32 { unsafe { if dwStreamNumber == 1 { * (lpData as * mut i64) = StreamBytesTransferred ; } c :: PROGRESS_CONTINUE } } let mut size = 0i64 ; cvt (unsafe { c :: CopyFileExW (from . as_ptr () , to . as_ptr () , Some (callback) , (& raw mut size) as * mut _ , ptr :: null_mut () , 0 ,) }) ? ; Ok (size as u64) }
}

macro_rules! junction_point_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function junction_point in module {}", module_path!());
    };
}

mkfn!{
    junction_point_introspect!();
    pub fn junction_point (original : & Path , link : & Path) -> io :: Result < () > { let mut opts = OpenOptions :: new () ; opts . create_new (true) ; opts . write (true) ; opts . custom_flags (c :: FILE_FLAG_BACKUP_SEMANTICS | c :: FILE_FLAG_POSIX_SEMANTICS) ; opts . attributes (c :: FILE_ATTRIBUTE_DIRECTORY) ; let d = File :: open (link , & opts) ? ; let path_bytes = original . as_os_str () . as_encoded_bytes () ; let abs_path : Vec < u16 > = if path_bytes . starts_with (br"\\?\") || path_bytes . starts_with (br"\??\") { let bytes = unsafe { OsStr :: from_encoded_bytes_unchecked (& path_bytes [4 ..]) } ; r"\??\" . encode_utf16 () . chain (bytes . encode_wide ()) . collect () } else { let abs_path = crate :: path :: absolute (original) ? . into_os_string () . into_encoded_bytes () ; if abs_path . len () > 0 && abs_path [1 ..] . starts_with (br":\") { let bytes = unsafe { OsStr :: from_encoded_bytes_unchecked (& abs_path) } ; r"\??\" . encode_utf16 () . chain (bytes . encode_wide ()) . collect () } else if abs_path . starts_with (br"\\.\") { let bytes = unsafe { OsStr :: from_encoded_bytes_unchecked (& abs_path [4 ..]) } ; r"\??\" . encode_utf16 () . chain (bytes . encode_wide ()) . collect () } else if abs_path . starts_with (br"\\") { let bytes = unsafe { OsStr :: from_encoded_bytes_unchecked (& abs_path [2 ..]) } ; r"\??\UNC\" . encode_utf16 () . chain (bytes . encode_wide ()) . collect () } else { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "path is not valid")) ; } } ; # [repr (C)] pub struct MountPointBuffer { ReparseTag : u32 , ReparseDataLength : u16 , Reserved : u16 , SubstituteNameOffset : u16 , SubstituteNameLength : u16 , PrintNameOffset : u16 , PrintNameLength : u16 , PathBuffer : [MaybeUninit < u16 > ; c :: MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize] , } let data_len = 12 + (abs_path . len () * 2) ; if data_len > u16 :: MAX as usize { return Err (io :: const_error ! (io :: ErrorKind :: InvalidInput , "`original` path is too long")) ; } let data_len = data_len as u16 ; let mut header = MountPointBuffer { ReparseTag : c :: IO_REPARSE_TAG_MOUNT_POINT , ReparseDataLength : data_len , Reserved : 0 , SubstituteNameOffset : 0 , SubstituteNameLength : (abs_path . len () * 2) as u16 , PrintNameOffset : ((abs_path . len () + 1) * 2) as u16 , PrintNameLength : 0 , PathBuffer : [MaybeUninit :: uninit () ; c :: MAXIMUM_REPARSE_DATA_BUFFER_SIZE as usize] , } ; unsafe { let ptr = header . PathBuffer . as_mut_ptr () ; ptr . copy_from (abs_path . as_ptr () . cast_uninit () , abs_path . len ()) ; let mut ret = 0 ; cvt (c :: DeviceIoControl (d . as_raw_handle () , c :: FSCTL_SET_REPARSE_POINT , (& raw const header) . cast :: < c_void > () , data_len as u32 + 8 , ptr :: null_mut () , 0 , & mut ret , ptr :: null_mut () ,)) . map (drop) } }
}

macro_rules! exists_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exists in module {}", module_path!());
    };
}

mkfn!{
    exists_introspect!();
    pub fn exists (path : & WCStr) -> io :: Result < bool > { let mut opts = OpenOptions :: new () ; opts . access_mode (0) ; opts . custom_flags (c :: FILE_FLAG_BACKUP_SEMANTICS) ; match File :: open_native (path , & opts) { Err (e) => match e . kind () { io :: ErrorKind :: NotFound => Ok (false) , _ if e . raw_os_error () == Some (c :: ERROR_SHARING_VIOLATION as i32) => Ok (true) , _ if e . raw_os_error () == Some (c :: ERROR_CANT_ACCESS_FILE as i32) => Ok (true) , _ => Err (e) , } , Ok (_) => Ok (true) , } }
}
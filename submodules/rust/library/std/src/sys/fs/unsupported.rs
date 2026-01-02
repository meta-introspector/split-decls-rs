mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: fs :: TryLockError ;}
mkuse!{use crate :: hash :: { Hash , Hasher } ;}
mkuse!{use crate :: io :: { self , BorrowedCursor , IoSlice , IoSliceMut , SeekFrom } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: sys :: time :: SystemTime ;}
mkuse!{use crate :: sys :: unsupported ;}
mkitem!{mkstruct!{pub struct File (!) ;}}
mkitem!{mkstruct!{pub struct FileAttr (!) ;}}
mkitem!{mkstruct!{pub struct ReadDir (!) ;}}
mkitem!{mkstruct!{pub struct DirEntry (!) ;}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct OpenOptions { }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct FileTimes { }}}
mkitem!{mkstruct!{pub struct FilePermissions (!) ;}}
mkitem!{mkstruct!{pub struct FileType (!) ;}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DirBuilder { }}}
mkitem!{mkimpl!{impl FileAttr { pub fn size (& self) -> u64 { self . 0 } pub fn perm (& self) -> FilePermissions { self . 0 } pub fn file_type (& self) -> FileType { self . 0 } pub fn modified (& self) -> io :: Result < SystemTime > { self . 0 } pub fn accessed (& self) -> io :: Result < SystemTime > { self . 0 } pub fn created (& self) -> io :: Result < SystemTime > { self . 0 } }}}
mkitem!{mkimpl!{impl Clone for FileAttr { fn clone (& self) -> FileAttr { self . 0 } }}}
mkitem!{mkimpl!{impl FilePermissions { pub fn readonly (& self) -> bool { self . 0 } pub fn set_readonly (& mut self , _readonly : bool) { self . 0 } }}}
mkitem!{mkimpl!{impl Clone for FilePermissions { fn clone (& self) -> FilePermissions { self . 0 } }}}
mkitem!{mkimpl!{impl PartialEq for FilePermissions { fn eq (& self , _other : & FilePermissions) -> bool { self . 0 } }}}
mkitem!{mkimpl!{impl Eq for FilePermissions { }}}
mkitem!{mkimpl!{impl fmt :: Debug for FilePermissions { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkimpl!{impl FileTimes { pub fn set_accessed (& mut self , _t : SystemTime) { } pub fn set_modified (& mut self , _t : SystemTime) { } }}}
mkitem!{mkimpl!{impl FileType { pub fn is_dir (& self) -> bool { self . 0 } pub fn is_file (& self) -> bool { self . 0 } pub fn is_symlink (& self) -> bool { self . 0 } }}}
mkitem!{mkimpl!{impl Clone for FileType { fn clone (& self) -> FileType { self . 0 } }}}
mkitem!{mkimpl!{impl Copy for FileType { }}}
mkitem!{mkimpl!{impl PartialEq for FileType { fn eq (& self , _other : & FileType) -> bool { self . 0 } }}}
mkitem!{mkimpl!{impl Eq for FileType { }}}
mkitem!{mkimpl!{impl Hash for FileType { fn hash < H : Hasher > (& self , _h : & mut H) { self . 0 } }}}
mkitem!{mkimpl!{impl fmt :: Debug for FileType { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkimpl!{impl fmt :: Debug for ReadDir { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}
mkitem!{mkimpl!{impl Iterator for ReadDir { type Item = io :: Result < DirEntry > ; fn next (& mut self) -> Option < io :: Result < DirEntry > > { self . 0 } }}}
mkitem!{mkimpl!{impl DirEntry { pub fn path (& self) -> PathBuf { self . 0 } pub fn file_name (& self) -> OsString { self . 0 } pub fn metadata (& self) -> io :: Result < FileAttr > { self . 0 } pub fn file_type (& self) -> io :: Result < FileType > { self . 0 } }}}
mkitem!{mkimpl!{impl OpenOptions { pub fn new () -> OpenOptions { OpenOptions { } } pub fn read (& mut self , _read : bool) { } pub fn write (& mut self , _write : bool) { } pub fn append (& mut self , _append : bool) { } pub fn truncate (& mut self , _truncate : bool) { } pub fn create (& mut self , _create : bool) { } pub fn create_new (& mut self , _create_new : bool) { } }}}
mkitem!{mkimpl!{impl File { pub fn open (_path : & Path , _opts : & OpenOptions) -> io :: Result < File > { unsupported () } pub fn file_attr (& self) -> io :: Result < FileAttr > { self . 0 } pub fn fsync (& self) -> io :: Result < () > { self . 0 } pub fn datasync (& self) -> io :: Result < () > { self . 0 } pub fn lock (& self) -> io :: Result < () > { self . 0 } pub fn lock_shared (& self) -> io :: Result < () > { self . 0 } pub fn try_lock (& self) -> Result < () , TryLockError > { self . 0 } pub fn try_lock_shared (& self) -> Result < () , TryLockError > { self . 0 } pub fn unlock (& self) -> io :: Result < () > { self . 0 } pub fn truncate (& self , _size : u64) -> io :: Result < () > { self . 0 } pub fn read (& self , _buf : & mut [u8]) -> io :: Result < usize > { self . 0 } pub fn read_vectored (& self , _bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 } pub fn is_read_vectored (& self) -> bool { self . 0 } pub fn read_buf (& self , _cursor : BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 } pub fn write (& self , _buf : & [u8]) -> io :: Result < usize > { self . 0 } pub fn write_vectored (& self , _bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 } pub fn is_write_vectored (& self) -> bool { self . 0 } pub fn flush (& self) -> io :: Result < () > { self . 0 } pub fn seek (& self , _pos : SeekFrom) -> io :: Result < u64 > { self . 0 } pub fn size (& self) -> Option < io :: Result < u64 > > { self . 0 } pub fn tell (& self) -> io :: Result < u64 > { self . 0 } pub fn duplicate (& self) -> io :: Result < File > { self . 0 } pub fn set_permissions (& self , _perm : FilePermissions) -> io :: Result < () > { self . 0 } pub fn set_times (& self , _times : FileTimes) -> io :: Result < () > { self . 0 } }}}
mkitem!{mkimpl!{impl DirBuilder { pub fn new () -> DirBuilder { DirBuilder { } } pub fn mkdir (& self , _p : & Path) -> io :: Result < () > { unsupported () } }}}
mkitem!{mkimpl!{impl fmt :: Debug for File { fn fmt (& self , _f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 } }}}

macro_rules! readdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readdir in module {}", module_path!());
    };
}

mkfn!{
    readdir_introspect!();
    pub fn readdir (_p : & Path) -> io :: Result < ReadDir > { unsupported () }
}

macro_rules! unlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unlink in module {}", module_path!());
    };
}

mkfn!{
    unlink_introspect!();
    pub fn unlink (_p : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (_old : & Path , _new : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! set_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_perm in module {}", module_path!());
    };
}

mkfn!{
    set_perm_introspect!();
    pub fn set_perm (_p : & Path , perm : FilePermissions) -> io :: Result < () > { match perm . 0 { } }
}

macro_rules! rmdir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rmdir in module {}", module_path!());
    };
}

mkfn!{
    rmdir_introspect!();
    pub fn rmdir (_p : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (_path : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! exists_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exists in module {}", module_path!());
    };
}

mkfn!{
    exists_introspect!();
    pub fn exists (_path : & Path) -> io :: Result < bool > { unsupported () }
}

macro_rules! readlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function readlink in module {}", module_path!());
    };
}

mkfn!{
    readlink_introspect!();
    pub fn readlink (_p : & Path) -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (_original : & Path , _link : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link in module {}", module_path!());
    };
}

mkfn!{
    link_introspect!();
    pub fn link (_src : & Path , _dst : & Path) -> io :: Result < () > { unsupported () }
}

macro_rules! stat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stat in module {}", module_path!());
    };
}

mkfn!{
    stat_introspect!();
    pub fn stat (_p : & Path) -> io :: Result < FileAttr > { unsupported () }
}

macro_rules! lstat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lstat in module {}", module_path!());
    };
}

mkfn!{
    lstat_introspect!();
    pub fn lstat (_p : & Path) -> io :: Result < FileAttr > { unsupported () }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (_p : & Path) -> io :: Result < PathBuf > { unsupported () }
}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    pub fn copy (_from : & Path , _to : & Path) -> io :: Result < u64 > { unsupported () }
}